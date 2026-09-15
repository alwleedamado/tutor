# Tutorial Sandbox — Vue State Management

This directory contains **educational artifacts only**.

## The isolation contract

1. Nothing here is imported by the application runtime.
2. `vuex` is NOT a dependency in `package.json` and must never be added.
3. These files are rendered as *readable code* inside lessons; they are never
   executed by Rust Mastery Offline.
4. If you want to actually run them, copy this folder into a scratch Vue
   project OUTSIDE this repository and install dependencies there.

| Folder      | Contents                                            |
|-------------|-----------------------------------------------------|
| `pinia/`    | Modern reference implementation (what the app uses) |
| `vuex/`     | Legacy Vuex 4 equivalents for reading               |
| `migration/`| Step-by-step Vuex → Pinia conversion guide          |

Why teach legacy at all? Because real jobs maintain pre-2021 codebases.
Reading fluency in Vuex — and the ability to argue Pinia's design from
experience — is a career skill. Building WITH it would be malpractice.
