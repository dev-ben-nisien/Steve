# Use an official Python runtime as a base image
FROM python:3.9-slim

# Install system dependencies (Git is needed for running git diff)
RUN apt-get update && apt-get install -y --no-install-recommends \
    git \
    && rm -rf /var/lib/apt/lists/*

# Set the working directory to GitHub's default workspace location
WORKDIR /github/workspace

COPY . ./Steve

RUN pip install --upgrade pip setuptools wheel && \
    pip install --no-cache-dir -r ./Steve/requirements.txt

CMD ["bash"]
