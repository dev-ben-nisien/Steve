# 🚀 Steve

**Steve** is a GitHub Actions tool that leverages large language models (LLMs) to analyze your architectural documentation for potential gaps and improvements. It automatically comments on pull requests with actionable feedback!

---

## ✨ Features

- **Architectural Analysis:** Detects issues and gaps in your docs using LLMs.
- **GitHub Integration:** Automatically creates or updates PR comments with analysis results.
- **Easy Setup:** Integrates seamlessly with your repository via GitHub Actions.

---

## 🛠️ Workflow Inputs

| Input             | Type   | Required | Description                                          |
| ----------------- | ------ | -------- | ---------------------------------------------------- |
| `github_token`    | String | Yes      | Your GitHub token for authenticating API calls.      |
| `openai_api_token`| String | Yes      | Your OpenAI API token to access LLM capabilities.      |
| `docs_path`       | String | Yes      | Path to the documentation file or folder to analyze. |

---

## ⚙️ Workflow Breakdown

1. **Set Up Environment:**  
   - Uses [actions/setup-python@v4](https://github.com/actions/setup-python) to install Python.

2. **Cache Dependencies:**  
   - Employs [actions/cache@v4](https://github.com/actions/cache) to cache pip packages.

3. **Install Dependencies:**  
   - Upgrades `pip`, `setuptools`, and `wheel` before installing packages from `requirements.txt`.

4. **Run Analysis:**  
   - Compares changes from `origin/main` and pipes them into `main.py` for analysis.
   - Captures output for later steps.

5. **Post PR Comment:**  
   - Utilizes [peter-evans/create-or-update-comment@v2](https://github.com/peter-evans/create-or-update-comment) to post your analysis as a comment on the pull request.

---

## 🚀 Setup Instructions

1. **Create the Workflow File:**  
   Save the YAML content as `.github/workflows/asksteve.yml` in your repository.

2. **Configure Secrets/Input Values:**  
   Define the following in your repository's settings or workflow call:
   - `github_token`
   - `openai_api_token`
   - `docs_path`

3. **Add Project Files:**  
   Ensure that your repository includes:
   - A `requirements.txt` to install dependencies.
   - A `main.py` that implements the architectural analysis logic.

---
