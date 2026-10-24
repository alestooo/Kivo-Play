import type {
  ClientMessage,
  ConnectionStatus,
  ServerMessage
} from './messages';

type MessageHandler = (message: ServerMessage) => void;
type StatusHandler = (status: ConnectionStatus) => void;

const DEFAULT_URL = 'ws://127.0.0.1:3000/ws';

export class KivoWebSocket {
  private socket: WebSocket | null = null;
  private connectingPromise: Promise<void> | null = null;
  private messageHandler: MessageHandler | null = null;
  private statusHandler: StatusHandler | null = null;
  private intentionalClose = false;

  connect(
    onMessage: MessageHandler,
    onStatus: StatusHandler,
    url: string = DEFAULT_URL
  ): Promise<void> {
    this.messageHandler = onMessage;
    this.statusHandler = onStatus;

    if (this.socket?.readyState === WebSocket.OPEN) {
      this.setStatus('connected');
      return Promise.resolve();
    }

    if (this.connectingPromise) {
      return this.connectingPromise;
    }

    this.intentionalClose = false;
    this.setStatus('connecting');

    this.connectingPromise = new Promise<void>((resolve, reject) => {
      const socket = new WebSocket(url);
      this.socket = socket;

      socket.onopen = () => {
        this.connectingPromise = null;
        this.setStatus('connected');
        resolve();
      };

      socket.onmessage = (event: MessageEvent) => {
        if (typeof event.data !== 'string') {
          return;
        }

        try {
          const message = JSON.parse(event.data) as ServerMessage;
          this.messageHandler?.(message);
        } catch (error) {
          console.error('Failed to parse Kivo WebSocket message:', error);
        }
      };

      socket.onerror = () => {
        this.setStatus('error');
      };

      socket.onclose = () => {
        const wasConnecting = this.connectingPromise !== null;

        this.connectingPromise = null;
        this.socket = null;

        if (this.intentionalClose) {
          this.setStatus('disconnected');
          return;
        }

        this.setStatus('disconnected');

        if (wasConnecting) {
          reject(new Error('WebSocket connection failed.'));
        }
      };
    });

    return this.connectingPromise;
  }

  send(message: ClientMessage): boolean {
    if (!this.socket || this.socket.readyState !== WebSocket.OPEN) {
      return false;
    }

    this.socket.send(JSON.stringify(message));
    return true;
  }

  isConnected(): boolean {
    return this.socket?.readyState === WebSocket.OPEN;
  }

  close(): void {
    this.intentionalClose = true;
    this.connectingPromise = null;

    if (
      this.socket &&
      (this.socket.readyState === WebSocket.OPEN ||
        this.socket.readyState === WebSocket.CONNECTING)
    ) {
      this.socket.close(1000, 'Client left');
    }

    this.socket = null;
    this.setStatus('disconnected');
  }

  private setStatus(status: ConnectionStatus): void {
    this.statusHandler?.(status);
  }
}
