<script lang="ts">
import FilterField from "./lib/FilterField.svelte";

let inputValues = {};

function resetInputValues() {
  inputValues = {};
  document.getElementById('segment_filter').value = '';
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
    body: "MSH|^~\\&|MEDAT|LaborBerlin|1003389|Charite|20230310115510||ORU^R01|MSG79850001-00000147|P|2.3|1003389\n" +
            "PID|1||65882306||6406df1f40b1e^6406df1f40b1d||20230307|0|||^^12167Berlin^D|||||||||||||\n" +
            "PV1|1|1|PROBATIX||||||||||||||||7036526637|1003389\n" +
            "ORC|RE|79850001|||CM||||20230310111112\n" +
            "OBR|1|79850001||79850001/79850001||9400|20230307065200|202303101155|||||||68||||||79850001|||Extern|F||^^^20230310111112^^0||\n" +
            "OBX|1|ST|400^kleines Blutbild (EDTA)^Medat||X||||||F|||20230310115130|Extern||IM/FCM|alna||kleines Blutbild (EDTA)||||Extern\n" +
            "OBX|2|NM|401^Leukozyten^Medat||10.00|/nl|9.00-28.20||||F|||20230310115128|Extern||Impedanz|alna||Leukozyten||||Extern\n" +
            "OBX|3|NM|402^Erythrozyten^Medat||6.0|/pl|3.9-6.3||||F|||20230310115130|Extern||Impedanz|alna||Erythrozyten||||Extern\n" +
            "OBX|4|NM|415^Hämoglobin^Medat||15.0|g/dl|13.2-21.7||||F|||20230310115132|Extern||Photom.|alna||Hämoglobin||||Extern\n" +
            "OBX|5|NM|416^Hämatokrit (l/l)^Medat||0.410|l/l|0.410-0.660||||F|||20230310115136|Extern||Berechng|alna||Hämatokrit (l/l)||||Extern\n" +
            "OBX|6|NM|405^MCV^Medat||100.0|fl|91.0-124.0||||F|||20230310115139|Extern||Impedanz|alna||MCV||||Extern\n" +
            "OBX|7|NM|406^MCH^Medat||35.0|pg|30.0-39.5||||F|||20230310115355|Extern||Berechng|alna||MCH||||Extern\n" +
            "OBX|8|NM|407^MCHC^Medat||30.0|g/dl|29.0-36.0||||F|||20230310115356|Extern||Berechng|alna||MCHC||||Extern\n" +
            "OBX|9|NM|413^RDW-CV^Medat||15.0|%|11.5-15.0||||F|||20230310115358|Extern||Berechng|alna||RDW-CV||||Extern\n" +
            "OBX|10|NM|408^Thrombozyten^Medat||220|/nl|220-520||||F|||20230310115400|Extern||Impedanz|alna||Thrombozyten||||Extern\n" +
            "OBX|11|NM|1^Natrium Se^Medat||150|mmol/l|131-144|H|||F|||20230310115142|Extern||ind.ISE|alna||Natrium Se||||Extern\n" +
            "OBX|12|NM|2^Kalium Se^Medat||3.0|mmol/l|3.2-5.7|L|||F|||20230310115145|Extern||ind.ISE|alna||Kalium Se||||Extern\n" +
            "OBX|13|NM|101^Harnstoff Se^Medat||50|mg/dl|||||F|||20230310115214|Extern||kinTest|alna||Harnstoff Se||||Extern\n" +
            "OBX|14|NM|102^Kreatinin (Jaffé) Se^Medat||1.00|mg/dl|0.29-1.04||||F|||20230310115151|Extern||Jaffe|alna||Kreatinin (Jaffé) Se||||Extern\n" +
            "OBX|15|ST|1639^geschätzte GFR (eGFR n Schwartz)^Medat||n.berechenbar|ml/min|||||F|||20230310115151|Extern||Berechng|alna||geschätzte GFR (eGFR n Schwartz)||||Extern\n" +
            "OBX|16|NM|50^GOT (AST) Se^Medat||20|U/l|16-96||||F|||20230310115155|Extern||Photom.|alna||GOT (AST) Se||||Extern\n" +
            "OBX|17|NM|51^GPT (ALT) Se^Medat||20|U/l|||||F|||20230310115223|Extern||Photom.|alna||GPT (ALT) Se||||Extern\n" +
            "OBX|18|NM|52^gamma-GT Se^Medat||20|U/l|||||F|||20230310115224|Extern||Photom.|alna||gamma-GT Se||||Extern\n" +
            "OBX|19|NM|53^Alk.Phosphatase Se^Medat||20|U/l|||||F|||20230310115225|Extern||Photom.|alna||Alk.Phosphatase Se||||Extern\n" +
            "OBX|20|NM|104^Harnsäure Se^Medat||10.0|mg/dl|||||F|||20230310115230|Extern||eFbt|alna||Harnsäure Se||||Extern\n" +
            "OBX|21|NM|105^Bilirubin gesamt Se^Medat||5.00|mg/dl|< 8.00||||F|||20230310115243|Extern||DPD-Farb|alna||Bilirubin gesamt Se||||Extern\n" +
            "OBX|22|NM|160^ges.Cholesterin Se^Medat||100|mg/dl|< 190||||F|||20230310115245|Extern||eFbt|alna||ges.Cholesterin Se||||Extern\n" +
            "NTE|1||               Referenzbereich angepasst nach:\n" +
            "NTE|2||               Mach F., et. al.: 2019 ESC/EAS Guidelines for the management of\n" +
            "NTE|3||               dyslipidaemias: lipid modification to reduce cardiovascular risk.\n" +
            "OBX|23|NM|162^HDL-Cholesterin Se^Medat||50|mg/dl|||||F|||20230310115246|Extern||eFbt|alna||HDL-Cholesterin Se||||Extern\n" +
            "NTE|1||               Referenzbereich angepasst nach:\n" +
            "NTE|2||               Mach F., et. al.: 2019 ESC/EAS Guidelines for the management of\n" +
            "NTE|3||               dyslipidaemias: lipid modification to reduce cardiovascular risk.\n" +
            "OBX|24|NM|165^non-HDL Cholesterin^Medat||50|mg/dl|||||F|||20230310115246|Extern||Berechng|alna||non-HDL Cholesterin||||Extern\n" +
            "OBX|25|NM|163^LDL-Cholesterin Se^Medat||60|mg/dl|< 116||||F|||20230310115256|Extern||Photom.|alna||LDL-Cholesterin Se||||Extern\n" +
            "NT\n" +
            "NTE|2||               Mach F., et. al.: 2019 ESC/EAS Guidelines for the management of\n" +
            "NTE|3||               dyslipidaemias: lipid modification to reduce cardiovascular risk.\n" +
            "OBX|26|NM|161^Triglyceride Se^Medat||10|mg/dl|||||F|||20230310115304|Extern||Photom.|alna||Triglyceride Se||||Extern\n" +
            "NTE|1||               Referenzbereich angepasst nach:\n" +
            "NTE|2||               Mach F., et. al.: 2019 ESC/EAS Guidelines for the management of\n" +
            "NTE|3||               dyslipidaemias: lipid modification to reduce cardiovascular risk.\n" +
            "NTE|4||      <150 mg/dl (nüchtern)\n" +
            "NTE|5||      <175 mg/dl (nicht nüchtern)\n" +
            "OBX|27|NM|900^Glucose im Fluorid^Medat||70|mg/dl|60-110||||F|||20230310115308|Extern||Hexo|alna||Glucose im Fluorid||||Extern\n" +
            "OBX|28|NM|801^TSH basal Se^Medat||5.00|mU/l|||||F|||20230310115316|Extern||ECLIA|alna|frpe13|TSH basal Se||||Extern\n" +
            "OBX|29|NM|850^25-OH-Vitamin D3 Se^Medat||100.0|nmol/l|75.0-175.0||||F|||20230310115318|Extern|||alna|frpe13|25-OH-Vitamin D3 Se||||Extern\n" +
            "NTE|1||25-Vitamin D:\n" +
            "NTE|2||< 25 nmol/l klinisch relevanter Vit.-D Mangel\n" +
            "NTE|3||25 - 50 nmol/l subklinischer Vit.-D Mangel\n" +
            "NTE|4||> 50 nmol/l gute/normale Vit.-D Versorgung\n" +
            "OBX|30|TX|8571^Endokrinologie validiert von:^^|    0|||||||F|||20230310115450|1532|||frpe13|frpe13|Endokrinologie validiert von:||\n" +
            "NTE|1||Dr. Frank Holger Perschel\n" +
            "NTE|2||Facharzt für Laboratoriumsmedizin\n" +
            "OBX|31|NM|813^Vitamin B12 i.Serum^Medat||200|ng/l|191-663||||F|||20230310115310|Extern||ECLIA|alna||Vitamin B12 i.Serum||||Extern\n" +
            "OBX|32|NM|5991^Holotranscobalamin i.Se^Medat||40.0|pmol/l|37.5-188.0||||F|||20230310115313|Extern|||alna||Holotranscobalamin i.Se||||Extern"
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
    let x = convertToCSV(segments);
    document.getElementById('results').innerText = x;
  })
}
</script>

<main>
  <FilterField label="Segment Filter" id="segment_filter" onInput="{handleInputChange}"></FilterField>
  <FilterField label="Field Filter" id="field_filter" onInput="{handleInputChange}"></FilterField>
  <button type="button" on:click={resetInputValues}>Reset</button>
  <div id="results"></div>
</main>

<style>
</style>
