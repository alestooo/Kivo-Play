<script lang="ts">
  type ServerMessage =
    | { type: 'pong' }
    | { type: 'error'; message: string };

  let status = 'disconnected';
  let response = '-';

  let socket: WebSocket | null = null;

  function connect() {
    if (
      socket?.readyState === WebSocket.OPEN ||
      socket?.readyState === WebSocket.CONNECTING
    ) {
      return;
    }

    status = 'connecting';

    socket = new WebSocket('ws://127.0.0.1:3000/ws');

    socket.addEventListener('open', () => {
      status = 'connected';
    });

    socket.addEventListener('message', (event) => {
      try {
        const message = JSON.parse(event.data) as ServerMessage;

        if (message.type === 'pong') {
          response = 'pong';
        }

        if (message.type === 'error') {
          response = message.message;
        }
      } catch {
        response = 'invalid server message';
      }
    });

    socket.addEventListener('close', () => {
      status = 'disconnected';
      socket = null;
    });

    socket.addEventListener('error', () => {
      status = 'error';
    });
  }

  function sendPing() {
    if (socket?.readyState !== WebSocket.OPEN) {
      return;
    }

    socket.send(
      JSON.stringify({
        type: 'ping'
      })
    );
  }
</script>

<section>
  <h2>WebSocket JSON Test</h2>

  <p>Status: {status}</p>
  <p>Server response: {response}</p>

  <button onclick={connect}>
    Connect
  </button>

  <button onclick={sendPing} disabled={status !== 'connected'}>
    Send Ping
  </button>
</section>