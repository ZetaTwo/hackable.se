import Tag from './Tag.vue'

<template>
  <table>
  <caption>Scoreboard</caption>
  <thead>
    <tr>
      <th>Name</th>
      <th>Category</th>
      <th>Tags</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td data-label="Name">A nice challenge</td>
      <td data-label="Category">Pwnable</td>
      <td data-label="Tags"><Tag tag="lol"><mark class="tag">pwn</mark><mark class="tag">crypto</mark></td>
    </tr>
    <tr>
      <td data-label="Name">A cool challenge</td>
      <td data-label="Category">Rev</td>
      <td data-label="Tags">Pwnable</td>
    </tr>
    <tr>
      <td data-label="Name">A difficult challenge</td>
      <td data-label="Category">Crypto</td>
      <td data-label="Tags">Pwnable</td>
    </tr>
  </tbody>
</table>
</template>

<script>
export default {
    
}
</script>

<style scoped>

</style>
