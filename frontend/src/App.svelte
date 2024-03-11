<script lang="ts">
import FilterField from "./lib/FilterField.svelte";

let inputValues = {};

function resetInputValues() {
  inputValues = {};
  // document.getElementById('segment_filter').value = '';
  document.getElementById('field_filter').value = '';
  query()
}
function handleInputChange(event) {
  inputValues[event.target.id] = event.target.value;
  query()
}

function query() {
  let queryString = new URLSearchParams(inputValues).toString();
  let url = 'http://127.0.0.1:8060/message?' + queryString
  fetch(url, {
    method: 'POST',
    headers: {
      'Content-Type': 'plain/text'
    },
    body: document.getElementById('request_body').value
  }).then(async (response) => {
    let segments = await response.json();
    function convertToCSV(data) {
      let csvContent = "";

      data.forEach(function (rowArray) {
        let row = rowArray.map(obj => '"' + obj.content + '"').join(",");
        csvContent += row + "\r\n";
      });

      return csvContent;
    }
    let csvContent = convertToCSV(segments);
    document.getElementById('results').innerText = csvContent;
  })
}
</script>

<main>
  <div class="container">
    <form class="item">
<!--      <FilterField label="Segment Filter" id="segment_filter" onInput="{handleInputChange}"></FilterField>-->
      <FilterField label="Field Filter" id="field_filter" onInput="{handleInputChange}"></FilterField>
      <label for="request_body">Hl7v2 Message</label>
      <textarea id="request_body"></textarea>
      <button type="button" on:click={resetInputValues}>Reset</button>
    </form>
    <div class="item" id="results"></div>
  </div>
</main>

<style>
</style>
