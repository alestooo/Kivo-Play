import type {
  ClientMessage,
  ConnectionStatus,
  ServerMessage
} from './messages';

type MessageHandler = (
  message: ServerMessage
) => void;

type StatusHandler = (
  status: ConnectionStatus
) => void;

const DEFAULT_WS_URL =
  'ws://127.0.0.1:3000/ws';

export class KivoWebSocket {
  private socket: WebSocket | null =
    null;

  private messageHandler:
    | MessageHandler
    | null = null;

  private statusHandler:
    | StatusHandler
    | null = null;

  connect(
    onMessage: MessageHandler,
    onStatus: StatusHandler
  ): Promise<void> {
    this.messageHandler = onMessage;
    this.statusHandler = onStatus;

    if (
      this.socket?.readyState ===
      WebSocket.OPEN
    ) {
      onStatus('connected');

      return Promise.resolve();
    }

    if (
      this.socket?.readyState ===
      WebSocket.CONNECTING
    ) {
      return this.waitForConnection();
    }

    onStatus('connecting');

    return new Promise(
      (resolve, reject) => {
        this.socket =
          new WebSocket(
            DEFAULT_WS_URL
          );

        this.socket.addEventListener(
          'open',
          () => {
            this.statusHandler?.(
              'connected'
            );

            resolve();
          }
        );

        this.socket.addEventListener(
          'message',
          (event) => {
            try {
              const message =
                JSON.parse(
                  event.data
                ) as ServerMessage;

              this.messageHandler?.(
                message
              );
            } catch {
              this.messageHandler?.({
                type: 'error',
                message:
                  'Invalid server response.'
              });
            }
          }
        );

        this.socket.addEventListener(
          'close',
          () => {
            this.statusHandler?.(
              'disconnected'
            );

            this.socket = null;
          }
        );

        this.socket.addEventListener(
          'error',
          () => {
            this.statusHandler?.(
              'error'
            );

            reject(
              new Error(
                'WebSocket connection failed.'
              )
            );
          }
        );
      }
    );
  }

  send(
    message: ClientMessage
  ): boolean {
    if (
      this.socket?.readyState !==
      WebSocket.OPEN
    ) {
      return false;
    }

    this.socket.send(
      JSON.stringify(message)
    );

    return true;
  }

  close(): void {
    this.socket?.close();

    this.socket = null;
  }

  isConnected(): boolean {
    return (
      this.socket?.readyState ===
      WebSocket.OPEN
    );
  }

  private waitForConnection():
    Promise<void> {
    return new Promise(
      (resolve, reject) => {
        const check = () => {
          if (
            this.socket?.readyState ===
            WebSocket.OPEN
          ) {
            this.statusHandler?.(
              'connected'
            );

            resolve();

            return;
          }

          if (
            this.socket?.readyState ===
              WebSocket.CLOSED ||
            this.socket?.readyState ===
              WebSocket.CLOSING
          ) {
            reject(
              new Error(
                'WebSocket connection failed.'
              )
            );

            return;
          }

          setTimeout(check, 100);
        };

        check();
      }
    );
  }
}