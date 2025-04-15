"""
Log viewer component for the Zappy GUI client
"""
from PyQt6.QtWidgets import (QWidget, QVBoxLayout, QHBoxLayout, QLabel, 
                            QTextEdit, QPushButton, QComboBox, QCheckBox,
                            QGroupBox, QSplitter)
from PyQt6.QtCore import Qt, pyqtSignal
from PyQt6.QtGui import QColor, QTextCursor, QFont, QTextCharFormat

import time


class LogViewer(QWidget):
    """Panel for displaying server logs and game events"""
    
    def __init__(self):
        super().__init__()
        
        # Log settings
        self.max_log_entries = 1000  # Maximum number of log entries to keep
        self.auto_scroll = True      # Auto-scroll to bottom
        
        # Log categories and colors
        self.log_categories = {
            "connection": QColor(120, 180, 255),   # Blue
            "player": QColor(255, 160, 120),       # Orange
            "resource": QColor(160, 255, 120),     # Green
            "incantation": QColor(255, 120, 255),  # Purple
            "egg": QColor(255, 255, 120),          # Yellow
            "broadcast": QColor(120, 255, 255),    # Cyan
            "game": QColor(255, 255, 255),         # White
            "error": QColor(255, 100, 100),        # Red
        }
        
        self.init_ui()
    
    def init_ui(self):
        """Initialize the UI components"""
        layout = QVBoxLayout(self)
        
        # Controls
        controls_layout = QHBoxLayout()
        
        # Log filter
        self.filter_combo = QComboBox()
        self.filter_combo.addItem("All Events")
        for category in self.log_categories.keys():
            self.filter_combo.addItem(category.capitalize())
        
        controls_layout.addWidget(QLabel("Filter:"))
        controls_layout.addWidget(self.filter_combo)
        
        # Auto-scroll checkbox
        self.auto_scroll_checkbox = QCheckBox("Auto-scroll")
        self.auto_scroll_checkbox.setChecked(self.auto_scroll)
        self.auto_scroll_checkbox.stateChanged.connect(self.toggle_auto_scroll)
        controls_layout.addWidget(self.auto_scroll_checkbox)
        
        # Clear button
        self.clear_button = QPushButton("Clear")
        self.clear_button.clicked.connect(self.clear_logs)
        controls_layout.addWidget(self.clear_button)
        
        layout.addLayout(controls_layout)
        
        # Log display area
        self.log_text = QTextEdit()
        self.log_text.setReadOnly(True)
        self.log_text.setLineWrapMode(QTextEdit.LineWrapMode.WidgetWidth)
        
        # Set monospace font for better log readability
        font = QFont("Courier New", 10)
        self.log_text.setFont(font)
        
        # Dark theme for log area
        self.log_text.setStyleSheet("background-color: #1e1e1e; color: #d4d4d4;")
        
        layout.addWidget(self.log_text, stretch=1)
    
    def toggle_auto_scroll(self, state):
        """Toggle auto-scroll feature"""
        self.auto_scroll = state == Qt.CheckState.Checked.value
    
    def clear_logs(self):
        """Clear all logs"""
        self.log_text.clear()
    
    def add_log(self, message, category="game"):
        """Add a log entry with timestamp and category color"""
        # Format timestamp
        timestamp = time.strftime("%H:%M:%S")
        
        # Create format for this category
        fmt = QTextCharFormat()
        
        # Set color based on category
        color = self.log_categories.get(category.lower(), QColor(255, 255, 255))
        fmt.setForeground(color)
        
        # Create the log entry
        full_message = f"[{timestamp}] {message}"
        
        # Add to log
        cursor = self.log_text.textCursor()
        cursor.movePosition(QTextCursor.MoveOperation.End)
        
        # Apply the format
        cursor.insertText(full_message + "\n", fmt)
        
        # Auto-scroll to bottom if enabled
        if self.auto_scroll:
            self.log_text.setTextCursor(cursor)
            self.log_text.ensureCursorVisible()
        
        # Limit number of entries (remove oldest if too many)
        document = self.log_text.document()
        if document.blockCount() > self.max_log_entries:
            cursor = QTextCursor(document)
            cursor.movePosition(QTextCursor.MoveOperation.Start)
            cursor.movePosition(QTextCursor.MoveOperation.Down, 
                              QTextCursor.MoveMode.KeepAnchor, 
                              document.blockCount() - self.max_log_entries)
            cursor.removeSelectedText()
    
    def add_connection_log(self, message):
        """Add a connection log entry"""
        self.add_log(message, "connection")
    
    def add_player_log(self, message):
        """Add a player log entry"""
        self.add_log(message, "player")
    
    def add_resource_log(self, message):
        """Add a resource log entry"""
        self.add_log(message, "resource")
    
    def add_incantation_log(self, message):
        """Add an incantation log entry"""
        self.add_log(message, "incantation")
    
    def add_egg_log(self, message):
        """Add an egg log entry"""
        self.add_log(message, "egg")
    
    def add_broadcast_log(self, message):
        """Add a broadcast log entry"""
        self.add_log(message, "broadcast")
    
    def add_error_log(self, message):
        """Add an error log entry"""
        self.add_log(message, "error")