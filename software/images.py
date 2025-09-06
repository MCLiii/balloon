import os
import pygame
from PIL import Image

# Ensure fbcon (framebuffer) driver is used
os.environ.setdefault("SDL_VIDEODRIVER", "fbcon")
os.environ.setdefault("SDL_FBDEV", "/dev/fb0")
os.environ.setdefault("SDL_NOMOUSE", "1")

pygame.display.init()
screen = pygame.display.set_mode((0, 0), pygame.FULLSCREEN)  # use native res
width, height = screen.get_size()

# Load with PIL (better format support), scale to screen
img = Image.open("test.png").convert("RGB").resize((width, height))
# Convert PIL -> pygame surface
mode = img.mode
data = img.tobytes()
surface = pygame.image.fromstring(data, (width, height), mode)

# Blit + show
screen.blit(surface, (0, 0))
pygame.display.flip()

# Keep it on screen until a keypress (or adjust to your needs)
while True:
    for event in pygame.event.get():
        if event.type == pygame.KEYDOWN:
            pygame.quit()
            raise SystemExit
