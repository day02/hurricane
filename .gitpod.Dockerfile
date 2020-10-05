FROM gitpod/workspace-full-vnc
USER root
RUN apt update && apt install -y libsdl2-dev libsdl2-image-dev libsdl2-ttf-dev