"""
Server connection for the Zappy GUI client
"""
import socket
import threading
import queue
import time
import logging

# Set up logging
logging.basicConfig(level=logging.INFO, format='%(asctime)s - %(levelname)s - %(message)s')
logger = logging.getLogger()


class ServerConnection:
    """Handles connection and communication with the Zappy server"""
    
    def __init__(self, host, port):
        """Initialize a server connection with the given host and port"""
        self.host = host
        self.port = port
        self.socket = None
        self.connected = False
        self.on_status_change = None  # Callback
        self.response_queue = queue.Queue()
        self.receive_thread = None
        self.running = False
    
    def connect(self):
        """Connect to the server"""
        try:
            self.socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
            self.socket.settimeout(2)  # Add timeout for connection attempts
            self.socket.connect((self.host, self.port))
            self.socket.settimeout(None)  # Remove timeout for normal operation
            
            # Wait for welcome message
            welcome = self.socket.recv(4096).decode('utf-8').strip()
            if "WELCOME" not in welcome:
                logger.error(f"Invalid welcome message: {welcome}")
                self.disconnect()
                return False
            
            # Send "GRAPHIC" to identify as GUI client
            self.socket.sendall(b"GRAPHIC\n")
            time.sleep(0.2)
            
            # Start receive thread
            self.running = True
            self.connected = True
            self.on_status_change(self.connected)
            self.receive_thread = threading.Thread(target=self._receive_loop)
            self.receive_thread.daemon = True
            self.receive_thread.start()
            
            # Request initial game state
            self._request_initial_state()
            logger.info(f"Connected to server {self.host}:{self.port}")
            
            return True
        except socket.timeout:
            logger.error(f"Connection timed out to {self.host}:{self.port}")
            self.disconnect()
            return False
        except ConnectionRefusedError:
            logger.error(f"Connection refused to {self.host}:{self.port}")
            self.disconnect()
            return False
        except Exception as e:
            logger.error(f"Connection error: {e}", exc_info=True)
            self.disconnect()
            return False
    
    def _request_initial_state(self):
        """Request initial game state from server"""
        if self.connected:
            # Request map size
            self.send_command("msz")
            # Request all teams
            self.send_command("tna")
            # Request time unit
            self.send_command("sgt")
            # Request map content (all tiles)
            self.send_command("mct")
    
    def disconnect(self):
        """Disconnect from the server"""
        self.running = False
        self.connected = False
        self.on_status_change(self.connected)
        
        if self.socket:
            try:
                self.socket.close()
                logger.info("Socket closed")
            except Exception as e:
                logger.error(f"Error closing socket: {e}")
            self.socket = None

    
    def send_command(self, command):
        """Send a command to the server"""
        if not self.connected or not self.socket:
            logger.warning(f"Cannot send command: not connected ({command})")
            return False
        
        try:
            if not command.endswith('\n'):
                command += '\n'
            self.socket.sendall(command.encode('utf-8'))
            return True
        except ConnectionError as e:
            logger.error(f"Connection error while sending command: {e}")
            self.disconnect()
            return False
        except Exception as e:
            logger.error(f"Error sending command: {e}", exc_info=True)
            self.disconnect()
            return False
    
    def get_responses(self):
        """Get all available responses from the server"""
        responses = []
        try:
            while not self.response_queue.empty():
                responses.append(self.response_queue.get_nowait())
                self.response_queue.task_done()
        except queue.Empty:
            pass
        
        return responses
    
    def _receive_loop(self):
        """Background thread to receive data from the server"""
        buffer = ""
        
        while self.running and self.socket:
            try:
                data = self.socket.recv(4096)
                if not data:  # Connection closed
                    logger.info("Server closed connection")
                    self.disconnect()
                    break
                
                buffer += data.decode('utf-8')
                
                # Split by newlines and process complete messages
                lines = buffer.split('\n')
                buffer = lines.pop()  # Keep the last incomplete line in the buffer
                
                for line in lines:
                    if line:
                        # Add to queue for GUI processing
                        self.response_queue.put(line)
                        
            except ConnectionError as e:
                logger.error(f"Connection error in receive loop: {e}")
                self.disconnect()
                break
            except Exception as e:
                logger.error(f"Error in receive loop: {e}", exc_info=True)
                self.disconnect()
                break

    def send_new_time_unit(self, time_unit):
        """Send new time unit to server"""
        self.send_command(f"sst {time_unit}")
        self.send_command(f"sgt")
