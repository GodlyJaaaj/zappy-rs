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
logger = logging.getLogger("ZappyConnection")


class ServerConnection:
    """Handles connection and communication with the Zappy server"""
    
    def __init__(self, host, port):
        """Initialize a server connection with the given host and port"""
        self.host = host
        self.port = port
        self.socket = None
        self.connected = False
        self.response_queue = queue.Queue()
        self.receive_thread = None
        self.running = False
        
        # Track connected players and their levels for win condition monitoring
        self.players = {}  # player_id -> {'team': team_name, 'level': level}
        self.teams = {}    # team_name -> [player_ids]
        self.max_level_players = {}  # team_name -> count of level 8 players
    
    def connect(self):
        """Connect to the server"""
        try:
            self.socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
            self.socket.settimeout(10)  # Add timeout for connection attempts
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
            
            # Start receive thread
            self.running = True
            self.connected = True
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
        
        if self.socket:
            try:
                self.socket.close()
                logger.info("Socket closed")
            except Exception as e:
                logger.error(f"Error closing socket: {e}")
            self.socket = None
            
        # Only try to join the thread if it's not the current thread
        if self.receive_thread and self.receive_thread.is_alive() and self.receive_thread != threading.current_thread():
            self.receive_thread.join(timeout=1.0)
            if self.receive_thread.is_alive():
                logger.warning("Receive thread did not terminate properly")
            self.receive_thread = None
        else:
            self.receive_thread = None
    
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
                    if line:  # Skip empty lines
                        # Pre-process specific events to track win condition
                        self._preprocess_event(line)
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
    
    def _preprocess_event(self, event):
        """Pre-process events to track win condition data"""
        parts = event.split()
        if not parts:
            return
        
        cmd = parts[0]
        
        try:
            # New player connection
            if cmd == "pnw" and len(parts) >= 7:  # Fixed the condition: pnw has 7 parts
                # pnw #n X Y O L N - new player connection
                player_id = int(parts[1][1:])  # Remove the # prefix
                level = int(parts[5])
                team_name = parts[6]
                
                # Track player
                self.players[player_id] = {'team': team_name, 'level': level}
                
                # Add to team tracking
                if team_name not in self.teams:
                    self.teams[team_name] = []
                    self.max_level_players[team_name] = 0
                
                self.teams[team_name].append(player_id)
                
                # Check if this is a max level player (level 8)
                if level == 8:
                    self.max_level_players[team_name] += 1
                    
                    # Check win condition (6 players at level 8)
                    if self.max_level_players[team_name] >= 6:
                        # Queue special event for win condition
                        self.response_queue.put(f"win_condition {team_name}")
            
            # Player level change (after incantation)
            elif cmd == "plv" and len(parts) >= 3:
                # plv #n L - player's level
                player_id = int(parts[1][1:])
                new_level = int(parts[2])
                
                if player_id in self.players:
                    old_level = self.players[player_id]['level']
                    team_name = self.players[player_id]['team']
                    
                    # Update player level
                    self.players[player_id]['level'] = new_level
                    
                    # Check if player reached max level
                    if old_level < 8 and new_level == 8:
                        self.max_level_players[team_name] += 1
                        
                        # Check win condition (6 players at level 8)
                        if self.max_level_players[team_name] >= 6:
                            # Queue special event for win condition
                            self.response_queue.put(f"win_condition {team_name}")
            
            # Player death
            elif cmd == "pdi" and len(parts) >= 2:
                # pdi #n - death of a player
                player_id = int(parts[1][1:])
                
                if player_id in self.players:
                    team_name = self.players[player_id]['team']
                    level = self.players[player_id]['level']
                    
                    # Remove from team tracking
                    if team_name in self.teams and player_id in self.teams[team_name]:
                        self.teams[team_name].remove(player_id)
                    
                    # Update max level player count if needed
                    if level == 8 and team_name in self.max_level_players:
                        self.max_level_players[team_name] -= 1
                    
                    # Remove player from tracking
                    del self.players[player_id]
            
            # End of game
            elif cmd == "seg" and len(parts) >= 2:
                # seg N - end of game
                winning_team = parts[1]
                
                # Queue special event for win condition
                self.response_queue.put(f"win_condition {winning_team}")
                
        except Exception as e:
            logger.error(f"Error preprocessing event: {e} - Event: {event}", exc_info=True)
    
    def check_win_condition(self):
        """Check if any team has met the win condition (6+ players at level 8)"""
        for team_name, count in self.max_level_players.items():
            if count >= 6:
                return team_name
        return None