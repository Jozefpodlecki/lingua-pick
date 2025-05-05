# 🧱 Architecture Overview

## Session

A Session represents a single learning attempt by the user. It encapsulates:

- The selected **language**
- A list of **exercises** the user goes through
- Whether the session is **finished** or **canceled**
- **Timestamps** for creation, completion, updates, and cancellation
- The **total number of exercises** to be completed in the session

### Purpose

Sessions allow grouping a sequence of exercises under a coherent user experience, tracking progress over time and allowing analytics (e.g., time spent, accuracy, consistency).

### Schema