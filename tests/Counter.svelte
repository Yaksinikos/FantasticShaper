<script lang="ts">
  let count = 0;
  
  function increment() {
    count += 1;
  }
</script>

<div>
  <p>Count: {count}</p>
  <button on:click={increment}>Increment</button>
</div>