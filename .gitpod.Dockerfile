FROM gitpod/workspace-full-vnc

RUN sudo apt get update && sudo apt install -y libsdl2-dev libsdl2-image-dev libsdl2-ttf-dev