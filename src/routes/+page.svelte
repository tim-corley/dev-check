<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";

  type CheckResult = {
    name: string;
    installed: boolean;
    version: string;
  };

  let checkResults = $state<CheckResult[]>([]);

  async function runChecks() {
    checkResults = await invoke("doctor_check");
  }
</script>

<main class="container">
  <h1>Tauri Dev-Check</h1>

  <div class="row">
    <button onclick={runChecks}>Run Checks</button>
  </div>

  {#if checkResults.length > 0}
    <table>
      <thead>
        <tr>
          <th>Tool</th>
          <th>Status</th>
          <th>Version</th>
        </tr>
      </thead>
      <tbody>
        {#each checkResults as result}
          <tr>
            <td>{result.name}</td>
            <td>{result.installed ? "✅" : "❌"}</td>
            <td>{result.version}</td>
          </tr>
        {/each}
      </tbody>
    </table>
  {/if}
</main>

<style>
  table {
    width: 100%;
    margin-top: 2em;
    border-collapse: collapse;
  }

  th, td {
    border: 1px solid #ddd;
    padding: 8px;
  }

  th {
    background-color: #f2f2f2;
  }

  .container {
    margin: 0;
    padding-top: 10vh;
    display: flex;
    flex-direction: column;
    justify-content: center;
    text-align: center;
  }

  .row {
    display: flex;
    justify-content: center;
  }

  button {
    border-radius: 8px;
    border: 1px solid transparent;
    padding: 0.6em 1.2em;
    font-size: 1em;
    font-weight: 500;
    font-family: inherit;
    color: #0f0f0f;
    background-color: #ffffff;
    transition: border-color 0.25s;
    box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
  }

  button {
    cursor: pointer;
  }

  button:hover {
    border-color: #396cd8;
  }
  button:active {
    border-color: #396cd8;
    background-color: #e8e8e8;
  }

  @media (prefers-color-scheme: dark) {
    th {
      background-color: #2f2f2f;
    }

    td {
      border: 1px solid #555;
    }

    button {
      color: #ffffff;
      background-color: #0f0f0f98;
    }
    button:active {
      background-color: #0f0f0f69;
    }
  }
</style>
