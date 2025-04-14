"""
Game view for the Zappy GUI client - renders the game map
"""
from PyQt6.QtWidgets import QWidget, QGraphicsView, QGraphicsScene, QVBoxLayout, QLabel
from PyQt6.QtGui import (QColor, QBrush, QPen, QFont, QPolygonF, QPainter, 
                        QRadialGradient, QLinearGradient)
from PyQt6.QtCore import Qt, QPointF, QRectF, QTimer, pyqtSignal
import math
import random
import time


class GameView(QWidget):
    """Game view widget for rendering the Zappy game map"""
    
    def __init__(self):
        super().__init__()
        self.initUI()
        
        # Map properties
        self.map_width = 10  # default
        self.map_height = 10  # default
        self.cell_size = 60  # pixels
        
        # Game elements
        self.players = {}  # player_id -> player data
        self.eggs = {}  # egg_id -> egg data
        self.incantations = []  # list of active incantations
        self.tiles = {}  # (x,y) -> resources
        self.broadcasts = []  # Active broadcast animations
        
        # Colors for teams
        self.team_colors = {}
        self.color_index = 0
        self.predefined_colors = [
            QColor(255, 0, 0),    # Red
            QColor(0, 0, 255),    # Blue
            QColor(0, 255, 0),    # Green
            QColor(255, 165, 0),  # Orange
            QColor(128, 0, 128),  # Purple
            QColor(255, 192, 203),# Pink
            QColor(0, 255, 255),  # Cyan
            QColor(255, 255, 0),  # Yellow
        ]
        
        # Resource colors
        self.resource_colors = {
            0: QColor(240, 240, 160),  # Food (yellow)
            1: QColor(180, 180, 180),  # Linemate (gray)
            2: QColor(139, 69, 19),    # Deraumere (brown)
            3: QColor(0, 128, 128),    # Sibur (teal)
            4: QColor(255, 105, 180),  # Mendiane (pink)
            5: QColor(50, 205, 50),    # Phiras (green)
            6: QColor(138, 43, 226)    # Thystame (purple)
        }
        
        # Animation timer
        self.animation_timer = QTimer(self)
        self.animation_timer.timeout.connect(self.update_animations)
        self.animation_timer.start(50)  # 50ms = 20fps for animations
    
    def initUI(self):
        """Initialize the UI components"""
        layout = QVBoxLayout(self)
        layout.setContentsMargins(0, 0, 0, 0)
        
        # Create graphics view and scene
        self.scene = QGraphicsScene(self)
        self.view = QGraphicsView(self.scene)
        self.view.setRenderHint(QPainter.RenderHint.Antialiasing)
        self.view.setBackgroundBrush(QColor(50, 50, 50))  # Dark background
        
        layout.addWidget(self.view)
        
        # Info label at bottom
        self.info_label = QLabel("Map info: No data")
        layout.addWidget(self.info_label)
    
    def set_map_size(self, width, height):
        """Set or update the map size"""
        self.map_width = width
        self.map_height = height
        
        # Adjust cell size based on map dimensions
        view_width = self.view.viewport().width()
        view_height = self.view.viewport().height()
        
        self.cell_size = min(view_width / width, view_height / height) * 0.9
        
        # Update info
        self.info_label.setText(f"Map size: {width}x{height}")
        
        # Redraw everything
        self.redraw_map()
    
    def redraw_map(self):
        """Clear and redraw the entire map"""
        self.scene.clear()
        
        # Calculate map dimensions
        map_width_px = self.map_width * self.cell_size
        map_height_px = self.map_height * self.cell_size
        
        # Draw grid
        grid_pen = QPen(QColor(70, 70, 70))
        for i in range(self.map_width + 1):
            x = i * self.cell_size
            self.scene.addLine(x, 0, x, map_height_px, grid_pen)
        
        for i in range(self.map_height + 1):
            y = i * self.cell_size
            self.scene.addLine(0, y, map_width_px, y, grid_pen)
        
        # Draw tile contents
        for pos, resources in self.tiles.items():
            x, y = pos
            self.draw_tile(x, y, resources)
        
        # Draw eggs
        for egg_id, egg_data in self.eggs.items():
            self.draw_egg(egg_data)
        
        # Draw incantations
        for incantation in self.incantations:
            self.draw_incantation(incantation)
        
        # Draw players
        for player_id, player_data in self.players.items():
            self.draw_player(player_data)
        
        # Set scene rect and adjust view
        self.scene.setSceneRect(0, 0, map_width_px, map_height_px)
        self.view.fitInView(self.scene.sceneRect(), Qt.AspectRatioMode.KeepAspectRatio)
    
    def get_team_color(self, team_name):
        """Get a color for a team, creating one if needed"""
        if team_name not in self.team_colors:
            color = self.predefined_colors[self.color_index % len(self.predefined_colors)]
            self.team_colors[team_name] = color
            self.color_index += 1
        return self.team_colors[team_name]
    
    def draw_tile(self, x, y, resources):
        """Draw a tile with resources"""
        # Convert to pixel coordinates
        px = x * self.cell_size
        py = (self.map_height - y - 1) * self.cell_size  # Flip Y for visual consistency
        
        # Draw background
        tile_rect = QRectF(px, py, self.cell_size, self.cell_size)
        self.scene.addRect(tile_rect, QPen(Qt.GlobalColor.transparent), 
                           QBrush(QColor(30, 30, 30)))
        
        # Coordinates text
        coord_text = self.scene.addText(f"{x},{y}")
        coord_text.setDefaultTextColor(QColor(180, 180, 180))
        coord_text.setPos(px + 5, py + 5)
        coord_text.setFont(QFont("Arial", 8))
        
        # Draw resources
        if resources:
            resource_size = self.cell_size / 5
            spacing = resource_size * 1.2
            
            # Calculate how many resources to display (up to a reasonable limit)
            total_resources = sum(resources)
            max_display = 8  # Maximum resources to display per tile
            
            if total_resources > 0:
                # Position resources in a grid pattern
                grid_size = math.ceil(math.sqrt(max_display))
                
                displayed = 0
                for res_type, count in enumerate(resources):
                    for _ in range(min(count, max_display - displayed)):
                        if displayed >= max_display:
                            break
                            
                        # Calculate position in grid
                        grid_x = displayed % grid_size
                        grid_y = displayed // grid_size
                        
                        res_x = px + (grid_x + 1) * spacing
                        res_y = py + (grid_y + 1) * spacing
                        
                        # Draw resource
                        resource_rect = QRectF(res_x, res_y, resource_size, resource_size)
                        self.scene.addEllipse(resource_rect, 
                                             QPen(Qt.GlobalColor.transparent),
                                             QBrush(self.resource_colors.get(res_type, QColor(200, 200, 200))))
                        
                        displayed += 1
                    
                # If there are more resources than we can show, indicate with text
                if total_resources > max_display:
                    more_text = self.scene.addText(f"+{total_resources - max_display}")
                    more_text.setDefaultTextColor(QColor(255, 255, 255))
                    more_text.setPos(px + self.cell_size - 20, py + self.cell_size - 20)
    
    def draw_player(self, player_data):
        """Draw a player on the map"""
        player_id = player_data['id']
        x, y = player_data['x'], player_data['y']
        orientation = player_data['orientation']
        level = player_data.get('level', 1)
        team = player_data.get('team', 'unknown')
        
        # Convert to pixel coordinates
        px = (x + 0.5) * self.cell_size  # Center of cell
        py = (self.map_height - y - 0.5) * self.cell_size  # Flip Y, center of cell
        
        player_size = self.cell_size * 0.6
        
        # Base player shape (circle)
        player_rect = QRectF(px - player_size/2, py - player_size/2, player_size, player_size)
        
        # Create a radial gradient for the player
        team_color = self.get_team_color(team)
        gradient = QRadialGradient(px, py, player_size/2)
        gradient.setColorAt(0, team_color.lighter(150))
        gradient.setColorAt(1, team_color)
        
        player_circle = self.scene.addEllipse(player_rect, 
                                             QPen(Qt.GlobalColor.black, 1),
                                             QBrush(gradient))
        player_circle.setZValue(10)  # Draw players above other elements
        
        # Direction indicator (triangle)
        direction_size = player_size * 0.4
        
        # Calculate the points for the direction triangle based on orientation
        angle_rad = {1: math.pi/2, 2: 0, 3: -math.pi/2, 4: math.pi}[orientation]
        
        dx = math.cos(angle_rad) * direction_size
        dy = -math.sin(angle_rad) * direction_size  # Negative because Y is flipped in Qt
        
        triangle = QPolygonF()
        triangle.append(QPointF(px + dx, py + dy))
        triangle.append(QPointF(px + dy*0.5, py - dx*0.5))
        triangle.append(QPointF(px - dy*0.5, py + dx*0.5))
        
        direction_indicator = self.scene.addPolygon(triangle, 
                                                  QPen(Qt.GlobalColor.black, 1),
                                                  QBrush(QColor(255, 255, 255, 200)))
        direction_indicator.setZValue(11)
        
        # Player ID and level text
        id_text = self.scene.addText(f"#{player_id}")
        id_text.setDefaultTextColor(QColor(255, 255, 255))
        id_text.setPos(px - id_text.boundingRect().width()/2, 
                       py - player_size/2 - id_text.boundingRect().height())
        id_text.setZValue(12)
        
        level_text = self.scene.addSimpleText(f"L{level}")
        level_text.setBrush(QColor(255, 255, 200))
        level_text.setPos(px - level_text.boundingRect().width()/2, 
                          py + player_size/2)
        level_text.setZValue(12)
    
    def draw_egg(self, egg_data):
        """Draw an egg on the map"""
        egg_id = egg_data['id']
        x, y = egg_data['x'], egg_data['y']
        player_id = egg_data.get('player_id', 0)
        
        # Convert to pixel coordinates
        px = (x + 0.5) * self.cell_size  # Center of cell
        py = (self.map_height - y - 0.5) * self.cell_size  # Flip Y, center of cell
        
        egg_size = self.cell_size * 0.3
        
        # Draw egg (oval shape)
        egg_rect = QRectF(px - egg_size/2, py - egg_size/2, egg_size, egg_size*1.3)
        
        egg_item = self.scene.addEllipse(egg_rect, 
                                        QPen(QColor(100, 100, 100)),
                                        QBrush(QColor(240, 240, 200)))
        egg_item.setZValue(5)  # Above resources, below players
        
        # Egg ID text
        egg_text = self.scene.addText(f"e{egg_id}")
        egg_text.setDefaultTextColor(QColor(100, 100, 100))
        egg_text.setPos(px - egg_text.boundingRect().width()/2, 
                        py - egg_size - egg_text.boundingRect().height())
        egg_text.setZValue(6)
    
    def draw_incantation(self, incantation):
        """Draw an ongoing incantation"""
        x, y = incantation['x'], incantation['y']
        level = incantation['level']
        players = incantation.get('players', [])
        start_time = incantation.get('start_time', 0)
        
        # Convert to pixel coordinates
        px = (x + 0.5) * self.cell_size  # Center of cell
        py = (self.map_height - y - 0.5) * self.cell_size  # Flip Y, center of cell
        
        # Incantation circle radius adjusts with time for pulsating effect
        time_factor = (time.time() - start_time) % 1.0  # 0 to 1
        pulse = 0.8 + 0.2 * math.sin(time_factor * 2 * math.pi)  # 0.8 to 1.0
        
        incantation_radius = self.cell_size * 0.8 * pulse
        
        # Gradient for incantation circle
        gradient = QRadialGradient(px, py, incantation_radius)
        gradient.setColorAt(0, QColor(255, 255, 200, 100))  # Semi transparent center
        gradient.setColorAt(1, QColor(255, 100, 0, 150))    # Orange glow at edge
        
        # Draw incantation circle
        incantation_rect = QRectF(px - incantation_radius, py - incantation_radius, 
                                 incantation_radius * 2, incantation_radius * 2)
        
        incantation_item = self.scene.addEllipse(incantation_rect, 
                                               QPen(QColor(255, 165, 0, 200), 2),
                                               QBrush(gradient))
        incantation_item.setZValue(9)  # Below players but above other elements
        
        # Incantation text
        level_text = self.scene.addText(f"INCANTATION L{level}")
        level_text.setDefaultTextColor(QColor(255, 200, 0))
        level_text.setPos(px - level_text.boundingRect().width()/2, 
                         py - incantation_radius - level_text.boundingRect().height())
        level_text.setZValue(9)
    
    def update_animations(self):
        """Update any ongoing animations"""
        current_time = time.time()
        
        # Update broadcast animations
        broadcasts_to_remove = []
        for i, broadcast in enumerate(self.broadcasts):
            age = current_time - broadcast['time']
            if age > 2.0:  # Remove after 2 seconds
                broadcasts_to_remove.append(i)
        
        # Remove expired broadcasts
        for i in sorted(broadcasts_to_remove, reverse=True):
            self.broadcasts.pop(i)
        
        # Redraw incantations for pulsating effect
        if self.incantations:
            self.redraw_map()
    
    def resizeEvent(self, event):
        """Handle resize events to adjust the view"""
        super().resizeEvent(event)
        if hasattr(self, 'view') and hasattr(self, 'scene') and self.scene.sceneRect():
            self.view.fitInView(self.scene.sceneRect(), Qt.AspectRatioMode.KeepAspectRatio)
    
    # def reset_camera(self):
    #     """Reset the camera to fit the entire map in view"""
    #     if self.scene and self.scene.sceneRect():
    #         self.view.fitInView(self.scene.sceneRect(), Qt.AspectRatioMode.KeepAspectRatio)
    
    # Public methods to update the game state
    
    def update_tile(self, x, y, resources):
        """Update the resources on a tile"""
        self.tiles[(x, y)] = resources
        self.redraw_map()
    
    def add_player(self, player_id, x, y, orientation, level, team_name):
        """Add a new player to the map"""
        self.players[player_id] = {
            'id': player_id,
            'x': x,
            'y': y,
            'orientation': orientation,
            'level': level,
            'team': team_name
        }
        self.redraw_map()
    
    def update_player_position(self, player_id, x, y, orientation):
        """Update a player's position and orientation"""
        if player_id in self.players:
            self.players[player_id]['x'] = x
            self.players[player_id]['y'] = y
            self.players[player_id]['orientation'] = orientation
            self.redraw_map()
    
    def update_player_level(self, player_id, level):
        """Update a player's level"""
        if player_id in self.players:
            self.players[player_id]['level'] = level
            self.redraw_map()
    
    def remove_player(self, player_id):
        """Remove a player from the map"""
        if player_id in self.players:
            del self.players[player_id]
            self.redraw_map()
    
    def add_egg(self, egg_id, x, y, player_id):
        """Add a new egg to the map"""
        self.eggs[egg_id] = {
            'id': egg_id,
            'x': x,
            'y': y,
            'player_id': player_id,
            'time': time.time()
        }
        self.redraw_map()
    
    def remove_egg(self, egg_id):
        """Remove an egg from the map"""
        if egg_id in self.eggs:
            del self.eggs[egg_id]
            self.redraw_map()
    
    def start_incantation(self, x, y, level, players):
        """Start an incantation animation"""
        self.incantations.append({
            'x': x,
            'y': y,
            'level': level,
            'players': players,
            'start_time': time.time()
        })
        self.redraw_map()
    
    def end_incantation(self, x, y, success):
        """End an incantation animation"""
        # Find and remove the incantation
        for i, incantation in enumerate(self.incantations):
            if incantation['x'] == x and incantation['y'] == y:
                self.incantations.pop(i)
                break
        
        # Add a flash effect
        color = QColor(0, 255, 0, 150) if success else QColor(255, 0, 0, 150)
        self.flash_tile(x, y, color)
        
        self.redraw_map()
    
    def flash_tile(self, x, y, color):
        """Create a brief flash effect on a tile"""
        # Convert to pixel coordinates
        px = x * self.cell_size
        py = (self.map_height - y - 1) * self.cell_size
        
        # Create a rectangle for the flash
        flash_rect = QRectF(px, py, self.cell_size, self.cell_size)
        flash_item = self.scene.addRect(flash_rect, QPen(color, 2), QBrush(color))
        flash_item.setZValue(20)  # Above everything
        
        # Set up a timer to remove the flash
        QTimer.singleShot(500, lambda: self.scene.removeItem(flash_item))
    
    def show_broadcast(self, player_id, message):
        """Show a broadcast animation from a player"""
        if player_id in self.players:
            player = self.players[player_id]
            self.broadcasts.append({
                'player_id': player_id,
                'x': player['x'],
                'y': player['y'],
                'message': message,
                'time': time.time()
            })
            
            # Draw ripple animation
            self.draw_broadcast_ripple(player['x'], player['y'])
    
    def draw_broadcast_ripple(self, x, y):
        """Draw a broadcast ripple effect"""
        # Convert to pixel coordinates
        px = (x + 0.5) * self.cell_size
        py = (self.map_height - y - 0.5) * self.cell_size
        
        # Create a series of expanding circles that fade out
        for i in range(3):
            # Create delayed animations
            delay = i * 300  # ms
            
            # Start with a small circle
            QTimer.singleShot(delay, lambda c=i: self._create_ripple(px, py, c))
    
    def _create_ripple(self, x, y, circle_index):
        """Create a ripple circle effect for broadcast animation"""
        # Size based on the index (larger for later circles)
        size = self.cell_size * (1 + circle_index * 0.8)
        
        # Create circle with fading opacity
        ripple_rect = QRectF(x - size/2, y - size/2, size, size)
        
        # Gradient with transparency
        ripple_color = QColor(255, 255, 255, 180 - circle_index * 50)
        ripple_item = self.scene.addEllipse(ripple_rect, 
                                           QPen(ripple_color, 2),
                                           QBrush(Qt.GlobalColor.transparent))
        ripple_item.setZValue(15)
        
        # Animate expansion and fading
        def expand_and_fade(step, max_steps=20):
            if step >= max_steps:
                self.scene.removeItem(ripple_item)
                return
                
            # Expand size
            growth = 1.0 + step / 10.0
            new_size = size * growth
            new_rect = QRectF(x - new_size/2, y - new_size/2, new_size, new_size)
            ripple_item.setRect(new_rect)
            
            # Fade opacity
            opacity = 1.0 - step / max_steps
            new_color = QColor(255, 255, 255, int((180 - circle_index * 50) * opacity))
            ripple_item.setPen(QPen(new_color, 2))
            
            # Schedule next step
            QTimer.singleShot(50, lambda: expand_and_fade(step + 1, max_steps))
            
        # Start animation
        expand_and_fade(0)