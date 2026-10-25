import type {
  ClientMessage,
  ConnectionStatus,
  ServerMessage
} from './messages';

const LOCAL_WS_URL =
  'ws://127.0.0.1:3000/ws';

const PRODUCTION_WS_URL =
  'wss://kivo-play.onrender.com/ws';

const DEFAULT_URL =
  import.meta.env.DEV
    ? LOCAL_WS_URL
    : (
        import.meta.env.PUBLIC_WS_URL ??
        PRODUCTION_WS_URL
      );

export class KivoWebSocket {
  private socket:
    WebSocket | null =
      null;

  private onMessage:
    (
      message:
        ServerMessage
    ) => void =
      () => {};

  private onStatus:
    (
      status:
        ConnectionStatus
    ) => void =
      () => {};

  connect(
    onMessage:
      (
        message:
          ServerMessage
      ) => void,

    onStatus:
      (
        status:
          ConnectionStatus
      ) => void,

    url:
      string =
        DEFAULT_URL
  ): Promise<void> {
    this.onMessage =
      onMessage;

    this.onStatus =
      onStatus;

    if (
      this.socket &&
      (
        this.socket.readyState ===
          WebSocket.OPEN ||
        this.socket.readyState ===
          WebSocket.CONNECTING
      )
    ) {
      return Promise.resolve();
    }

    this.onStatus(
      'connecting'
    );

    return new Promise(
      (
        resolve,
        reject
      ) => {
        try {
          this.socket =
            new WebSocket(
              url
            );

          this.socket.onopen =
            () => {
              this.onStatus(
                'connected'
              );

              resolve();
            };

          this.socket.onmessage =
            (
              event
            ) => {
              try {
                const message =
                  JSON.parse(
                    event.data
                  ) as
                    ServerMessage;

                this.onMessage(
                  message
                );
              } catch (
                error
              ) {
                console.error(
                  'Invalid WebSocket message:',
                  error
                );
              }
            };

          this.socket.onerror =
            (
              event
            ) => {
              console.error(
                'WebSocket error:',
                event
              );

              this.onStatus(
                'error'
              );

              reject(
                new Error(
                  'WebSocket connection failed'
                )
              );
            };

          this.socket.onclose =
            () => {
              this.onStatus(
                'disconnected'
              );

              this.socket =
                null;
            };
        } catch (
          error
        ) {
          this.onStatus(
            'error'
          );

          reject(
            error
          );
        }
      }
    );
  }

  send(
    message:
      ClientMessage
  ): boolean {
    if (
      !this.socket ||
      this.socket.readyState !==
        WebSocket.OPEN
    ) {
      return false;
    }

    this.socket.send(
      JSON.stringify(
        message
      )
    );

    return true;
  }

  isConnected():
    boolean {
    return (
      this.socket
        ?.readyState ===
        WebSocket.OPEN
    );
  }

  close():
    void {
    if (!this.socket) {
      return;
    }

    this.socket.close();

    this.socket =
      null;
  }
}

export const GameWebSocket =
  KivoWebSocket;

export const connection =
  new KivoWebSocket();