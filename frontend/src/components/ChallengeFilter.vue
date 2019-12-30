<template>
    <form>
  <fieldset>
    <legend>Filter challenges</legend>
    <div class="row responsive-label">
      <div class="col-sm-12 col-md-3"><label for="search">Serch</label></div>
      <div class="col-sm-12 col-md"><input type="text" id="search" placeholder="Search"/></div>
    </div>
    <div class="row responsive-label">
      <div class="col-sm-12 col-md-3"><label for="category">Category</label></div>
      <div class="col-sm-12 col-md">
        <select id="category">
          <option valie="crypto">Cryptography</option>
          <option valie="rev">Reverse Engineering</option>
          <option valie="pwn">Pwnable</option>
          <option valie="for">Forensics</option>
          <option valie="misc">Miscellaneous</option>
          <option valie="web">Web</option>
        </select>
      </div>
    </div>
    <div class="row responsive-label">
      <div class="col-sm-12 col-md-3"><label for="tags">Tags</label></div>
      <div class="col-sm-12 col-md">
        <select id="tags">
          <option valie="heap">Heap</option>
          <option valie="rev">Reverse Engineering</option>
          <option valie="pwn">Pwnable</option>
          <option valie="for">Forensics</option>
          <option valie="misc">Miscellaneous</option>
          <option valie="web">Web</option>
        </select>
      </div>
    </div>
    <div class="row responsive-label">
      <div class="col-sm-12 col-md">
          <tag editable="true">heap</tag>
          <tag editable="true">rop</tag>
          <tag editable="true">crypto</tag>
      </div>
    </div>
  </fieldset>
</form>
</template>

<script>
import Tag from './Tag.vue'

export default {
    components: {
        Tag,
    }
}
</script>

<style scoped>

</style>
