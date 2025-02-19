# AskSteve

**AskSteve** is a tool that performs architectural analysis using large language models (LLMs) by checking your documentation for possible gaps and areas of improvement.

---

## Description

AskSteve uses LLMs to analyze your architectural documentation. Running within a GitHub Actions workflow, it:
- Scans the changes compared to your base branch.
- Runs architectural checks on your docs.
- Comments on pull requests if there’s any room for improvement.

---

## Features

- **Architectural Analysis:** Uses LLMs for detecting issues or gaps in your documentation.
- **GitHub Integration:** Automatically generates or updates PR comments based on analysis.
- **Easy Setup:** Leverages composite GitHub Actions steps for simple integration.

---

## Workflow Inputs

The workflow requires several inputs to work properly:

- **`github_token`** (`string`, required):  
  Your GitHub token to authenticate with the GitHub API.

- **`openai_api_token`** (`string`, required):  
  Your OpenAI API token used for accessing LLM capabilities.

- **`docs_path`** (`string`, required):  
  The path to your documentation file/folder which needs to be analyzed.

---

## Workflow Breakdown

### Branding

- **Icon:** 🚀 (rocket)
- **Color:** Blue

### Steps Overview

1. **Set up things:**  
   Uses [actions/setup-python@v4](https://github.com/actions/setup-python) to set up the Python environment.

2. **Cache Pip Dependencies:**  
   Caches pip dependencies to speed up successive builds using [actions/cache@v4](https://github.com/actions/cache).

3. **Install Dependencies:**  
   Upgrades `pip`, `setuptools`, and `wheel`, then installs all required packages from `requirements.txt`.

4. **Analyze Documentation (Do Stuff):**  
   - Checks for changes compared to `origin/main`.
   - Pipes those changes into `main.py`, which performs the analysis.
   - Captures the output for the next step.
   
5. **Create or Update PR Comment:**  
   Uses [peter-evans/create-or-update-comment@v2](https://github.com/peter-evans/create-or-update-comment) to post the analysis output as a comment on the current pull request.

---

## Setup

To add **AskSteve** to your repository:

1. **Define Workflow File:**  
   Save the provided YAML content to your repository inside the `.github/workflows` directory, e.g., `.github/workflows/asksteve.yml`.

2. **Provide Required Inputs:**  
   Ensure you have defined the following secrets/inputs in your repository or in the workflow call:
   - `github_token`
   - `openai_api_token`
   - `docs_path`

3. **Add `requirements.txt` and `main.py`:**  
   Ensure your project contains the `requirements.txt` for dependency installation and a `main.py` that implements the architectural analysis logic.

---

## Example Workflow YAML

Below is an example of how the GitHub Actions workflow is defined:

```yaml
name: "AskSteve"

description: "Does some architectural analysis using LLMs and says if your docs are lacking"
author: "BF"

branding:
  icon: "rocket"
  color: "blue"

inputs:
  github_token:
    type: string
    required: true
  openai_api_token:
    type: string
    required: true
  docs_path:
    type: string
    required: true

runs:
  using: "composite"
  steps:
    - name: Set up things
      uses: actions/setup-python@v4
      with:
        python-version: "3.x"
    - name: Is it cached?
      id: cache-pip
      uses: actions/cache@v4
      with:
        path: ~/.cache/pip
        key: ${{ runner.os }}-pip
    - name: Install
      shell: bash
      run: |
        pip install --upgrade pip setuptools wheel
        pip install -r requirements.txt
    - name: Do stuff
      id: run_script
      shell: bash
      run: |
          OUTPUT=$(git diff origin/main -- | python "main.py")
          echo "script_output<<EOF" >> "$GITHUB_OUTPUT"
          echo "$OUTPUT" >> "$GITHUB_OUTPUT"
          echo "EOF" >> "$GITHUB_OUTPUT"
      env:
        OPENAI_API_KEY: ${{ inputs.openai_api_token }}
        DOCS_PATH: ${{ inputs.docs_path }}
    - name: Create or Update PR Comment
      id: comment
      uses: peter-evans/create-or-update-comment@v2
      with:
        token: ${{ inputs.github_token }}
        issue-number: ${{ github.event.pull_request.number }}
        body: ${{ steps.run_script.outputs.script_output }}
