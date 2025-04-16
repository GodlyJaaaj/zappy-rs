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
    grid_changed = pyqtSignal(bool)
    coords_changed = pyqtSignal(bool)
    resources_changed = pyqtSignal(bool)
    text_size_changed = pyqtSignal(int)
    follow_player_changed = pyqtSignal(object)
    smooth_tracking_changed = pyqtSignal(bool)
    tracking_speed_changed = pyqtSignal(float)
    
    def __init__(self):
        super().__init__()

        self.setEnabled(False)

        self.text_size_value = None
        self.follow_player_combo = None
        self.show_resources_checkbox = None
        self.show_coords_checkbox = None
        self.show_grid_checkbox = None
        self.zoom_out_button = None
        self.map_size_label = None
        self.zoom_in_button = None
        self.apply_time_button = None
        self.time_input = None
        self.time_slider = None
        self.time_unit_label = None
        self.text_size_slider = None

        self.time_unit = 100  # Default time unit (frequency)
        self.map_width = 10   # Default map width
        self.map_height = 10  # Default map height
        
        self.init_ui()

    def set_enabled(self, enabled: bool):
        """Enable or disable all controls"""
        # Call parent setEnabled which handles all child widgets
        super().setEnabled(enabled)
    
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
        self.time_input = QSpinBox()
        self.time_input.setRange(1, 1000)
        self.time_input.setValue(self.time_unit)
        self.time_input.valueChanged.connect(self.on_time_spin_changed)
        
        time_slider_layout.addWidget(self.time_slider)
        time_slider_layout.addWidget(self.time_input)
        
        time_layout.addLayout(time_slider_layout)
        
        # Apply button for time unit
        self.apply_time_button = QPushButton("Apply Time Unit")
        self.apply_time_button.clicked.connect(self.apply_time_unit)
        time_layout.addWidget(self.apply_time_button)
        
        layout.addWidget(time_group)
        
        # View controls
        view_group = QGroupBox("View Controls")
        view_layout = QVBoxLayout(view_group)

        zoom_layout = QHBoxLayout()
        zoom_layout.addWidget(QLabel("Zoom:"))

        self.zoom_out_button = QPushButton("-")
        zoom_layout.addWidget(self.zoom_out_button)

        self.zoom_in_button = QPushButton("+")
        zoom_layout.addWidget(self.zoom_in_button)

        self.reset_zoom_button = QPushButton("Reset")
        zoom_layout.addWidget(self.reset_zoom_button)

        view_layout.addLayout(zoom_layout)
        
        # Display options
        display_layout = QVBoxLayout()

        # Existing code...
        self.show_grid_checkbox = QCheckBox("Show Grid")
        self.show_grid_checkbox.setChecked(True)
        self.show_grid_checkbox.stateChanged.connect(self.on_show_grid_changed)
        display_layout.addWidget(self.show_grid_checkbox)

        self.show_coords_checkbox = QCheckBox("Show Coordinates")
        self.show_coords_checkbox.setChecked(True)
        self.show_coords_checkbox.stateChanged.connect(self.on_show_coords_changed)
        display_layout.addWidget(self.show_coords_checkbox)

        self.show_resources_checkbox = QCheckBox("Show Resources")
        self.show_resources_checkbox.setChecked(False)
        self.show_resources_checkbox.setEnabled(False)
        self.show_resources_checkbox.stateChanged.connect(self.on_show_resources_changed)
        display_layout.addWidget(self.show_resources_checkbox)

        text_size_layout = QHBoxLayout()
        text_size_layout.addWidget(QLabel("Coordinates Size:"))

        self.text_size_slider = QSlider(Qt.Orientation.Horizontal)
        self.text_size_slider.setRange(4, 16)  # Font sizes from 4pt to 16pt
        self.text_size_slider.setValue(8)  # Default 8pt font
        self.text_size_slider.setTickPosition(QSlider.TickPosition.TicksBelow)
        self.text_size_slider.setTickInterval(2)
        self.text_size_slider.valueChanged.connect(self.on_text_size_changed)
        text_size_layout.addWidget(self.text_size_slider)

        self.text_size_value = QLabel("8")
        text_size_layout.addWidget(self.text_size_value)

        display_layout.addLayout(text_size_layout)
        
        view_layout.addLayout(display_layout)
        
        layout.addWidget(view_group)

        # Follow player controls
        follow_group = QGroupBox("Follow Player")
        follow_layout = QVBoxLayout(follow_group)

        self.follow_player_combo = QComboBox()
        self.follow_player_combo.addItem("None", None)
        self.follow_player_combo.currentIndexChanged.connect(self.on_follow_player_changed)
        follow_layout.addWidget(self.follow_player_combo)

        # Add smooth tracking checkbox
        self.smooth_tracking_checkbox = QCheckBox("Smooth Tracking")
        self.smooth_tracking_checkbox.setChecked(True)
        self.smooth_tracking_checkbox.stateChanged.connect(self.on_smooth_tracking_changed)
        follow_layout.addWidget(self.smooth_tracking_checkbox)

        tracking_speed_layout = QHBoxLayout()
        tracking_speed_layout.addWidget(QLabel("Speed:"))

        self.tracking_speed_slider = QSlider(Qt.Orientation.Horizontal)
        self.tracking_speed_slider.setRange(1, 50)  # 0.01 to 0.5
        self.tracking_speed_slider.setValue(10)  # 0.1 default
        self.tracking_speed_slider.valueChanged.connect(self.on_tracking_speed_changed)
        tracking_speed_layout.addWidget(self.tracking_speed_slider)

        follow_layout.addLayout(tracking_speed_layout)

        layout.addWidget(follow_group)
        
        # Add stretch to push controls to top
        layout.addStretch(1)

    def on_follow_player_changed(self, index):
        """Handle player selection changes"""
        player_id = self.follow_player_combo.currentData()
        self.follow_player_changed.emit(player_id)

    def update_player_list(self, players):
        """Update the player dropdown list"""
        current_id = self.follow_player_combo.currentData()

        self.follow_player_combo.blockSignals(True)
        self.follow_player_combo.clear()
        self.follow_player_combo.addItem("None", None)

        for player_id, player_data in players.items():
            team = player_data.get('team', 'unknown')
            level = player_data.get('level', 1)
            # Use a more concise format for player entries
            display_text = f"#{player_id} - {team} (L{level})"
            self.follow_player_combo.addItem(display_text, player_id)

        # Try to restore previous selection
        if current_id is not None:
            index = self.follow_player_combo.findData(current_id)
            if index != -1:
                self.follow_player_combo.setCurrentIndex(index)

        self.follow_player_combo.blockSignals(False)

    def on_text_size_changed(self, value):
        """Handle text size slider changes"""
        self.text_size_value.setText(str(value))
        self.text_size_changed.emit(value)

    def set_map_size(self, width, height):
        """Update the displayed map size"""
        self.map_width = width
        self.map_height = height
        self.map_size_label.setText(f"Map Size: {width}x{height}")

    def set_time_unit(self, time_unit):
        """Update the time unit display and controls"""
        self.time_unit = time_unit
        self.time_unit_label.setText(f"Time unit: {time_unit}")

        # Update slider and input without triggering signals
        self.time_slider.blockSignals(True)
        self.time_slider.setValue(time_unit)
        self.time_slider.blockSignals(False)

        self.time_input.blockSignals(True)
        self.time_input.setValue(time_unit)
        self.time_input.blockSignals(False)
    
    def on_time_slider_changed(self, value):
        """Handle time slider value changes"""
        self.time_unit = value
        self.time_unit_label.setText(f"Time unit: {value}")
        self.time_input.blockSignals(True)
        self.time_input.setValue(value)
        self.time_input.blockSignals(False)
    
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

    def on_show_grid_changed(self, state):
        """Handle show grid checkbox state changes"""
        self.grid_changed.emit(state == Qt.CheckState.Checked.value)

    def on_show_coords_changed(self, state):
        """Handle show coordinates checkbox state changes"""
        self.coords_changed.emit(state == Qt.CheckState.Checked.value)

    def on_show_resources_changed(self, state):
        """Handle show resources checkbox state changes"""
        self.resources_changed.emit(state == Qt.CheckState.Checked.value)

    def on_smooth_tracking_changed(self, state):
        """Handle smooth tracking checkbox state changes"""
        is_checked = state == Qt.CheckState.Checked.value
        self.smooth_tracking_changed.emit(is_checked)

    def on_tracking_speed_changed(self, value):
        """Handle tracking speed slider changes"""
        # Convert slider value (1-50) to speed value (0.01-0.5)
        speed = value / 100.0
        self.tracking_speed_changed.emit(speed)
