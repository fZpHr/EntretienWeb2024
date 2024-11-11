#!/bin/bash

# Ouvrir un terminal pour cargo watch -x run dans le répertoire axum_backend
gnome-terminal -- bash -c "cd axum_backend && cargo watch -x run; exec bash"

# Ouvrir un terminal pour npx tailwindcss -i ./css/input.css -o ./assets/tailwind.css --watch dans le répertoire dioxus_frontend
gnome-terminal -- bash -c "cd dioxus_frontend && npx tailwindcss -i input.css -o ./assets/tailwind.css --watch; exec bash"

# Ouvrir un terminal pour dx serve --hot-reload dans le répertoire dioxus_frontend
gnome-terminal -- bash -c "cd dioxus_frontend && dx serve --hot-reload; exec bash"
