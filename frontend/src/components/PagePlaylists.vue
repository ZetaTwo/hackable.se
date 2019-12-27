<template>
<div>
   <!-- TODO: separate into challenge search component -->
   <form>
  <fieldset>
    <legend>Filter playlists</legend>
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
        <mark class="tag">heap [x]</mark> 
        <mark class="tag">rop [x]</mark> 
        <mark class="tag">crypto [x]</mark> 
      </div>
    </div>
  </fieldset>
</form>
<div class="row">
    <!-- TODO: Extract challenge component -->
  <div class="card">
    <div class="section"><h3>Cool challenge</h3></div>
    <div class="section">Description of pretty cool challenge</div>
    <div class="section"><mark class="tag">pwn</mark> <mark class="tag">crypto</mark></div>
  </div>
  <div class="card">
    <div class="section"><h3>Diffcult challenge</h3></div>
    <div class="section">This one will test you</div>
    <div class="section"><mark class="tag">pwn</mark></div>
  </div>
  <div class="card">
    <div class="section"><h3>Diffcult challenge</h3></div>
    <div class="section">This one will test you</div>
    <div class="section"><mark class="tag">pwn</mark></div>
  </div>
  <div class="card">
    <div class="section"><h3>Diffcult challenge</h3></div>
    <div class="section">This one will test you</div>
    <div class="section"><mark class="tag">pwn</mark></div>
  </div>
  <div class="card">
    <div class="section"><h3>Diffcult challenge</h3></div>
    <div class="section">This one will test you</div>
    <div class="section"><mark class="tag">pwn</mark></div>
  </div>
  <div class="card">
    <div class="section"><h3>Diffcult challenge</h3></div>
    <div class="section">This one will test you</div>
    <div class="section"><mark class="tag">pwn</mark> <mark class="tag">heap</mark></div>
  </div>
</div>
</div>
</template>

<script>
export default {
    
}
</script>

<style scoped>

</style>
