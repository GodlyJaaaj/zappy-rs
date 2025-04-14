"""
Map controls panel for the Zappy GUI
"""
from PyQt6.QtWidgets import (QWidget, QVBoxLayout, QHBoxLayout, QLabel, 
                            QPushButton, QSlider, QSpinBox, QGroupBox, QComboBox,
                            QCheckBox)
from PyQt6.QtCore import Qt, pyqtSignal


class MapControls(QWidget):
    """Panel for map control operations and settings"""
    
    # Signals
    time_unit_changed = pyqtSignal(int)
    
    def __init__(self):
        super().__init__()
        
        self.time_unit = 100  # Default time unit (frequency)
        self.map_width = 10   # Default map width
        self.map_height = 10  # Default map height
        
        self.init_ui()
    
    def init_ui(self):
        """Initialize the UI components"""
        layout = QVBoxLayout(self)
        
        # Time unit control
        time_group = QGroupBox("Time Control")
        time_layout = QVBoxLayout(time_group)
        
        self.time_unit_label = QLabel(f"Time unit: {self.time_unit}")
        time_layout.addWidget(self.time_unit_label)
        
        time_slider_layout = QHBoxLayout()
        
        # Slider for time unit
        self.time_slider = QSlider(Qt.Orientation.Horizontal)
        self.time_slider.setRange(1, 1000)
        self.time_slider.setValue(self.time_unit)
        self.time_slider.setTickPosition(QSlider.TickPosition.TicksBelow)
        self.time_slider.setTickInterval(100)
        self.time_slider.valueChanged.connect(self.on_time_slider_changed)
        
        # Spin box for time unit
        self.time_spin = QSpinBox()
        self.time_spin.setRange(1, 1000)
        self.time_spin.setValue(self.time_unit)
        self.time_spin.valueChanged.connect(self.on_time_spin_changed)
        
        time_slider_layout.addWidget(self.time_slider)
        time_slider_layout.addWidget(self.time_spin)
        
        time_layout.addLayout(time_slider_layout)
        
        # Apply button for time unit
        self.apply_time_button = QPushButton("Apply Time Unit")
        self.apply_time_button.clicked.connect(self.apply_time_unit)
        time_layout.addWidget(self.apply_time_button)
        
        layout.addWidget(time_group)
        
        # Map information
        map_info_group = QGroupBox("Map Information")
        map_info_layout = QVBoxLayout(map_info_group)
        
        self.map_size_label = QLabel(f"Map Size: {self.map_width}x{self.map_height}")
        map_info_layout.addWidget(self.map_size_label)
        
        layout.addWidget(map_info_group)
        
        # View controls
        view_group = QGroupBox("View Controls")
        view_layout = QVBoxLayout(view_group)
        
        # Zoom controls
        zoom_layout = QHBoxLayout()
        zoom_layout.addWidget(QLabel("Zoom:"))
        
        self.zoom_out_button = QPushButton("-")
        zoom_layout.addWidget(self.zoom_out_button)
        
        self.zoom_in_button = QPushButton("+")
        zoom_layout.addWidget(self.zoom_in_button)
        
        view_layout.addLayout(zoom_layout)
        
        # Display options
        display_layout = QVBoxLayout()
        
        self.show_grid_checkbox = QCheckBox("Show Grid")
        self.show_grid_checkbox.setChecked(True)
        display_layout.addWidget(self.show_grid_checkbox)
        
        self.show_coords_checkbox = QCheckBox("Show Coordinates")
        self.show_coords_checkbox.setChecked(True)
        display_layout.addWidget(self.show_coords_checkbox)
        
        self.show_resources_checkbox = QCheckBox("Show Resources")
        self.show_resources_checkbox.setChecked(True)
        display_layout.addWidget(self.show_resources_checkbox)
        
        view_layout.addLayout(display_layout)
        
        layout.addWidget(view_group)
        
        # Follow player controls
        follow_group = QGroupBox("Follow Player")
        follow_layout = QVBoxLayout(follow_group)
        
        self.follow_player_combo = QComboBox()
        self.follow_player_combo.addItem("None")
        follow_layout.addWidget(self.follow_player_combo)
        
        layout.addWidget(follow_group)
        
        # Add stretch to push controls to top
        layout.addStretch(1)
    
    def set_map_size(self, width, height):
        """Update the displayed map size"""
        self.map_width = width
        self.map_height = height
        self.map_size_label.setText(f"Map Size: {width}x{height}")
    
    def set_time_unit(self, time_unit):
        """Update the time unit display"""
        self.time_unit = time_unit
        self.time_unit_label.setText(f"Time unit: {time_unit}")
        self.time_slider.setValue(time_unit)
        self.time_spin.setValue(time_unit)
    
    def on_time_slider_changed(self, value):
        """Handle time slider value changes"""
        self.time_unit = value
        self.time_unit_label.setText(f"Time unit: {value}")
        self.time_spin.blockSignals(True)
        self.time_spin.setValue(value)
        self.time_spin.blockSignals(False)
    
    def on_time_spin_changed(self, value):
        """Handle time spin box value changes"""
        self.time_unit = value
        self.time_unit_label.setText(f"Time unit: {value}")
        self.time_slider.blockSignals(True)
        self.time_slider.setValue(value)
        self.time_slider.blockSignals(False)
    
    def apply_time_unit(self):
        """Apply the selected time unit"""
        self.time_unit_changed.emit(self.time_unit)
    
    def add_player_to_follow(self, player_id, team_name):
        """Add a player to the follow combo box"""
        self.follow_player_combo.addItem(f"#{player_id} ({team_name})", player_id)
    
    def remove_player_from_follow(self, player_id):
        """Remove a player from the follow combo box"""
        index = self.follow_player_combo.findData(player_id)
        if index >= 0:
            self.follow_player_combo.removeItem(index)