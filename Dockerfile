FROM python:3.9-slim

RUN apt-get update && apt-get install -y --no-install-recommends \
    git \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /github/workspace

COPY requirements.txt ./Steve/requirements.txt

RUN pip install --no-cache-dir -r ./Steve/requirements.txt

CMD ["bash"]