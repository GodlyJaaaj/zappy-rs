#!/usr/bin/env python3
"""
Zappy GUI Client - Main Entry Point
"""
import sys

from PyQt6.QtGui import QGuiApplication
from PyQt6.QtWidgets import QApplication

if __package__ == '' or __package__ is None:
    from core.main_window import ZappyMainWindow
else:
    from gui.core.main_window import ZappyMainWindow


def main():
    """Main entry point for the application"""
    app = QApplication(sys.argv)
    # Use Fusion style which is available on all platforms
    app.setStyle('Fusion')

    window = ZappyMainWindow()

    window.show()
    
    sys.exit(app.exec())


if __name__ == "__main__":
    main()