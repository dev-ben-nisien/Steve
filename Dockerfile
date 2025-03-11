# Use an official Python runtime as a base image
FROM python:3.9-slim

# Install system dependencies (Git is needed for running git diff)
RUN apt-get update && apt-get install -y --no-install-recommends \
    git \
    && rm -rf /var/lib/apt/lists/*

# Set the working directory to GitHub's default workspace location
WORKDIR /github/workspace

# Copy your requirements file from the repository into the image.
# This assumes your repository contains a "Steve" folder with requirements.txt.
COPY requirements.txt ./Steve/requirements.txt

# Upgrade pip and install Python dependencies
RUN pip install --upgrade pip setuptools wheel && \
    pip install --no-cache-dir -r ./Steve/requirements.txt

# Optionally, copy the rest of your repository if you want them baked into
# your container image. In many GitHub Actions workflows the repo is mounted
# at runtime using actions/checkout so this may be omitted.
# COPY . .

# Default to an interactive shell so that the container can accept commands
CMD ["bash"]
