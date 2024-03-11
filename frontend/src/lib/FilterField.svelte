<script lang="ts">
    export let label: string = "";
    export let id: string = "";
    export let onInput;

</script>
<label for="{id}">{label}</label>
<input id="{id}" type="text" on:input={onInput}/>
