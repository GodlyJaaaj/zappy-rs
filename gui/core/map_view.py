"""
Game view for the Zappy GUI client - renders the game map
"""
import math

from PyQt6.QtCore import Qt, QPointF, QRectF
from PyQt6.QtGui import (QColor, QBrush, QPen, QFont, QPolygonF, QPainter,
                         QRadialGradient)
from PyQt6.QtWidgets import QWidget, QGraphicsView, QGraphicsScene, QVBoxLayout

from gui.core.player.PlayerManager import PlayerManager


class MapView(QWidget):
    """Game view widget for rendering the Zappy game map"""

    def clear(self):
        """Clear the map view"""
        self.scene.clear()
        self.grid_items = []
        self.coord_texts = {}
        self.resource_items = {}
        self.player_items = {}
        self.tiles = {}
        self.player_manager.clear()

    def __init__(self, player_manager: PlayerManager):
        self.grid_items = []
        self.coord_texts = {}
        self.resource_items = {}
        self.player_items = {}
        self.tiles = {}

        super().__init__()

        self.player_manager = player_manager
        self.tracked_player_id = None  # Add this line

        # Map properties
        self.map_width = 0  # default
        self.map_height = 0  # default
        self.cell_size = 60  # pixels

        # Display options
        self.view_initialized = None
        self.show_grid = True
        self.show_coordinates = True
        self.show_resources = True
        self.text_size = 8

        self.use_smooth_tracking = True  # Set to True by default
        self.tracking_speed = 0.1  # Default tracking speed
        
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
            "food": QColor(240, 240, 160),  # Food (yellow)
            "linemate": QColor(180, 180, 180),  # Linemate (gray)
            "deraumere": QColor(139, 69, 19),  # Deraumere (brown)
            "sibur": QColor(0, 128, 128),  # Sibur (teal)
            "mendiane": QColor(255, 105, 180),  # Mendiane (pink)
            "phiras": QColor(50, 205, 50),  # Phiras (green)
            "thystame": QColor(138, 43, 226)  # Thystame (purple)
        }

        self.resource_order = ["food", "linemate", "deraumere", "sibur", "mendiane", "phiras", "thystame"]

        # Components
        self.scene = None
        self.view = None
        self.tiles = {}

        self.initUI()

    def set_text_size(self, size):
        """Set the coordinate text size"""
        self.text_size = size
        self.draw_map()

    def initUI(self):
        """Initialize the UI components"""
        layout = QVBoxLayout(self)
        layout.setContentsMargins(0, 0, 0, 0)
        
        # Create graphics view and scene
        self.scene = QGraphicsScene(self)
        self.view = QGraphicsView(self.scene)
        self.view.setRenderHint(QPainter.RenderHint.Antialiasing)
        self.view.setBackgroundBrush(QColor(50, 50, 50))  # Dark background

        # Enable mouse interactions
        self.view.setDragMode(QGraphicsView.DragMode.ScrollHandDrag)  # Enable panning with mouse drag
        self.view.setTransformationAnchor(
            QGraphicsView.ViewportAnchor.AnchorUnderMouse)  # Zoom centers on mouse position
        self.view.setResizeAnchor(QGraphicsView.ViewportAnchor.AnchorUnderMouse)
        self.view.setVerticalScrollBarPolicy(Qt.ScrollBarPolicy.ScrollBarAlwaysOff)
        self.view.setHorizontalScrollBarPolicy(Qt.ScrollBarPolicy.ScrollBarAlwaysOff)

        # Allow scrolling beyond the scene boundaries for smoother navigation
        self.view.setSceneRect(-1000, -1000, 3000, 3000)  # Use large values to allow scrolling in all directions

        # Connect the wheel event for zooming
        self.view.wheelEvent = self.wheel_event

        layout.addWidget(self.view)

    def wheel_event(self, event):
        """Handle mouse wheel events for zooming"""
        zoom_factor = 1.15  # Zoom factor per wheel step

        # Zoom in or out depending on wheel direction
        if event.angleDelta().y() > 0:
            # Zoom in
            self.view.scale(zoom_factor, zoom_factor)
        else:
            # Zoom out
            self.view.scale(1.0 / zoom_factor, 1.0 / zoom_factor)

    def zoom_in(self):
        """Zoom in the view"""
        zoom_factor = 1.15
        self.view.scale(zoom_factor, zoom_factor)

    def zoom_out(self):
        """Zoom out the view"""
        zoom_factor = 1.15
        self.view.scale(1.0 / zoom_factor, 1.0 / zoom_factor)

    def reset_view(self):
        """Reset the view to fit the entire map"""
        self.view.fitInView(self.scene.sceneRect(), Qt.AspectRatioMode.KeepAspectRatio)
        self.view_initialized = True

    def set_map_size(self, width, height):
        """Set or update the map size"""
        self.map_width = width
        self.map_height = height
        
        # Adjust cell size based on map dimensions
        view_width = self.view.viewport().width()
        view_height = self.view.viewport().height()
        
        self.cell_size = min(view_width / width, view_height / height) * 0.9

        if not self.tiles:
            for x in range(width):
                for y in range(height):
                    self.tiles[(x, y)] = {res: 0 for res in self.resource_order}

    def draw_map(self):
        """Dessiner la carte initiale avec les éléments persistants"""
        self.scene.clear()
        self.grid_items = []
        self.coord_texts = {}
        self.resource_items = {}
        self.player_items = {}

        # Calculer les dimensions
        map_width_px = self.map_width * self.cell_size
        map_height_px = self.map_height * self.cell_size

        # Dessiner la grille (éléments persistants)
        if self.show_grid:
            grid_pen = QPen(QColor(255, 255, 255), 2.0)
            for i in range(self.map_width + 1):
                x = i * self.cell_size
                line = self.scene.addLine(x, 0, x, map_height_px, grid_pen)
                self.grid_items.append(line)

            for i in range(self.map_height + 1):
                y = i * self.cell_size
                line = self.scene.addLine(0, y, map_width_px, y, grid_pen)
                self.grid_items.append(line)

        # Dessiner les tuiles initiales
        for pos, resources in self.tiles.items():
            x, y = pos
            self.draw_tile_resources(x, y, resources)
            self.draw_coordinate_text(x, y)

        # Dessiner tous les joueurs
        for player_id, player_data in self.player_manager.all_players().items():
            self.draw_player(player_id, player_data)

        # Configurer la scène
        self.scene.setSceneRect(0, 0, map_width_px, map_height_px)

        if not hasattr(self, 'view_initialized') or not self.view_initialized:
            self.view.fitInView(self.scene.sceneRect(), Qt.AspectRatioMode.KeepAspectRatio)
            self.view_initialized = True

    def draw_coordinate_text(self, x, y):
        """Dessine le texte des coordonnées d'une tuile"""
        if not self.show_coordinates:
            return

        # Convertir en coordonnées de pixels
        px = x * self.cell_size
        py = (self.map_height - y - 1) * self.cell_size

        # Créer le texte des coordonnées
        coord_text = self.scene.addText(f"{x},{y}")
        coord_text.setDefaultTextColor(QColor(180, 180, 180))
        coord_text.setFont(QFont("monospace", self.text_size))

        # Calculer la position centrale du texte
        text_width = coord_text.boundingRect().width()
        text_height = coord_text.boundingRect().height()
        text_x = px + (self.cell_size - text_width) / 2
        text_y = py + (self.cell_size - text_height) / 2

        coord_text.setPos(text_x, text_y)

        # Stocker la référence
        self.coord_texts[(x, y)] = coord_text

    def draw_tile_resources(self, x, y, resources):
        """Dessine les ressources sur une tuile"""
        if not self.show_resources:
            return

    def draw_tile(self, x, y):
        """Met à jour une tuile avec ressources"""
        # Plus besoin de redessiner les coordonnées, car elles sont persistantes
        # Mettre à jour les ressources si nécessaire
        resources = self.tiles.get((x, y), {})

        # Si les ressources existent déjà visuellement, les mettre à jour
        # sinon les créer
        if (x, y) in self.resource_items:
            self.update_tile_resources_visual(x, y, resources)
        else:
            self.draw_tile_resources(x, y, resources)

    def update_tile_resources(self, x, y, resources):
        """Update resources on a specific tile

        Args:
            x: X coordinate
            y: Y coordinate
            resources: Dict with resource counts or list of resource counts in order
        """
        if (x, y) not in self.tiles:
            # Initialize the tile if it doesn't exist
            self.tiles[(x, y)] = {res: 0 for res in self.resource_order}

        if isinstance(resources, dict):
            # If resources is a dictionary, update individual resources
            for res_name, count in resources.items():
                if res_name in self.tiles[(x, y)]:
                    self.tiles[(x, y)][res_name] = count
        elif isinstance(resources, list):
            # If resources is a list, update in order of resource_order
            for i, res_name in enumerate(self.resource_order):
                if i < len(resources):
                    self.tiles[(x, y)][res_name] = resources[i]

    def draw_player(self, player_id, player_data):
        """Dessine ou met à jour un joueur sur la carte"""
        # Supprimer les anciens éléments graphiques si le joueur existe déjà
        if player_id in self.player_items:
            for item in self.player_items[player_id]:
                self.scene.removeItem(item)

        # Créer une liste pour stocker les nouveaux éléments graphiques
        player_graphics = []

        # Extraire les données du joueur
        position = player_data.get('position', (0, 0))
        x, y = position
        direction = player_data.get('direction', 'N')
        level = player_data.get('level', 1)
        team = player_data.get('team', 'default')

        # Convertir la direction en orientation numérique
        orientation_map = {"N": 1, "E": 2, "S": 3, "W": 4}
        orientation = orientation_map.get(direction, 1)

        # Convertir en coordonnées de pixels
        px = (x + 0.5) * self.cell_size
        py = (self.map_height - y - 0.5) * self.cell_size

        player_size = self.cell_size * 0.6

        # Forme de base du joueur (cercle)
        player_rect = QRectF(px - player_size / 2, py - player_size / 2, player_size, player_size)

        # Créer un dégradé radial pour le joueur
        team_color = self.get_team_color(team)
        gradient = QRadialGradient(px, py, player_size / 2)
        gradient.setColorAt(0, team_color.lighter(150))
        gradient.setColorAt(1, team_color)

        player_circle = self.scene.addEllipse(player_rect,
                                              QPen(Qt.GlobalColor.black, 1),
                                              QBrush(gradient))
        player_circle.setZValue(10)
        player_graphics.append(player_circle)

        # Indicateur de direction (triangle)
        direction_size = player_size * 0.4
        angle_rad = {1: math.pi / 2, 2: 0, 3: -math.pi / 2, 4: math.pi}[orientation]

        dx = math.cos(angle_rad) * direction_size
        dy = -math.sin(angle_rad) * direction_size

        triangle = QPolygonF()
        triangle.append(QPointF(px + dx, py + dy))
        triangle.append(QPointF(px + dy * 0.5, py - dx * 0.5))
        triangle.append(QPointF(px - dy * 0.5, py + dx * 0.5))

        direction_indicator = self.scene.addPolygon(triangle,
                                                    QPen(Qt.GlobalColor.black, 1),
                                                    QBrush(QColor(255, 255, 255, 200)))
        direction_indicator.setZValue(11)
        player_graphics.append(direction_indicator)

        # Textes d'ID et de niveau
        #id_text = self.scene.addText(f"#{player_id}")
        #id_text.setDefaultTextColor(QColor(255, 255, 255))
        #id_text.setPos(px - id_text.boundingRect().width() / 2,
        #               py - player_size / 2 - id_text.boundingRect().height())
        #id_text.setZValue(12)
        #player_graphics.append(id_text)

        #level_text = self.scene.addSimpleText(f"L{level}")
        #level_text.setBrush(QColor(255, 255, 200))
        #level_text.setPos(px - level_text.boundingRect().width() / 2,
        #                  py + player_size / 2)
        #level_text.setZValue(12)
        #player_graphics.append(level_text)

        # Stocker les références aux éléments graphiques
        self.player_items[player_id] = player_graphics

    def update_display(self):
        """Met à jour l'affichage en ne redessinant que les éléments nécessaires"""
        # Mettre à jour le suivi du joueur si nécessaire
        self.update_player_tracking()

        # Mettre à jour la visibilité de la grille si nécessaire
        for grid_item in self.grid_items:
            grid_item.setVisible(self.show_grid)

        # Mettre à jour la visibilité des coordonnées
        for coord_text in self.coord_texts.values():
            coord_text.setVisible(self.show_coordinates)

        # Mettre à jour la visibilité des ressources
        for resource_group in self.resource_items.values():
            for item in resource_group:
                item.setVisible(self.show_resources)

    def get_team_color(self, team_name):
        """Get a color for a team, creating one if needed"""
        if team_name not in self.team_colors:
            color = self.predefined_colors[self.color_index % len(self.predefined_colors)]
            self.team_colors[team_name] = color
            self.color_index += 1
        return self.team_colors[team_name]

    def center_on_player(self, player_id):
        """Center the view on the specified player"""
        if player_id is None or player_id not in self.player_manager.all_players():
            return

        player_data = self.player_manager.get(player_id)
        if player_data:
            x, y = player_data.get('position', (0, 0))

            # Convert to pixel coordinates (centered on the player)
            px = (x + 0.5) * self.cell_size
            py = (self.map_height - y - 0.5) * self.cell_size  # Y is flipped in the view

            # Center the view on this point
            self.view.centerOn(px, py)
            self.tracked_player_id = player_id

    def update_player_tracking(self):
        """Update camera position if tracking a player"""
        if self.tracked_player_id is None:
            return

        player_data = self.player_manager.get(self.tracked_player_id)
        if not player_data:
            return

        x, y = player_data.get('position', (0, 0))

        # Convert to pixel coordinates (centered on the player)
        target_px = (x + 0.5) * self.cell_size
        target_py = (self.map_height - y - 0.5) * self.cell_size  # Y is flipped in the view

        if hasattr(self, 'use_smooth_tracking') and self.use_smooth_tracking:
            # Get current view center
            view_center = self.view.mapToScene(self.view.viewport().rect().center())
            current_px = view_center.x()
            current_py = view_center.y()

            # Calculate distance to target
            distance = ((target_px - current_px) ** 2 + (target_py - current_py) ** 2) ** 0.5

            # Only move if there's a significant distance to avoid jitter
            if distance > 1.0:
                # Fixed speed factor (simpler but effective)
                speed = getattr(self, 'tracking_speed', 0.1)

                # Use exponential smoothing for more natural motion
                # This ensures smoother deceleration as we approach the target
                smoothing = 0.85  # Lower for more responsive, higher for smoother
                lerp_factor = min(1.0, speed * (1.0 - smoothing))

                new_px = current_px + (target_px - current_px) * lerp_factor
                new_py = current_py + (target_py - current_py) * lerp_factor

                # Use setViewportUpdateMode to optimize rendering
                previous_mode = self.view.viewportUpdateMode()
                self.view.setViewportUpdateMode(QGraphicsView.ViewportUpdateMode.FullViewportUpdate)

                # Center on the calculated position
                self.view.centerOn(new_px, new_py)

                # Restore previous viewport update mode
                self.view.setViewportUpdateMode(previous_mode)
        else:
            # Directly center on player
            self.view.centerOn(target_px, target_py)