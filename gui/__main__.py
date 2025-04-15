#!/usr/bin/env python3
"""
Zappy GUI Client - Main Entry Point
"""
import signal
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

    # Détecte les écrans disponibles
    screens = QGuiApplication.screens()
    if len(screens) > 1:
        second_screen = screens[1]
        geometry = second_screen.geometry()
        window.move(geometry.topLeft())

    window.show()
    
    sys.exit(app.exec())


if __name__ == "__main__":
    main()