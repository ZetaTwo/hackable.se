<template>
<form>
  <fieldset>
    <legend>Register account</legend>
    <div class="row responsive-label">
      <div class="col-sm-12 col-md-3"><label for="username">Username</label></div>
      <div class="col-sm-12 col-md"><input type="text" id="Username" placeholder="Username"/></div>
    </div>
    <div class="row responsive-label">
      <div class="col-sm-12 col-md-3"><label for="password">Password</label></div>
      <div class="col-sm-12 col-md"><input type="password" id="password" placeholder="Password"/></div>
    </div>
    <div class="row responsive-label">
      <div class="col-sm-12 col-md-3"><label for="password_confirm">Confirm password</label></div>
      <div class="col-sm-12 col-md"><input type="password" id="password_confirm" placeholder="Confirm password"/></div>
    </div>
    <div class="row responsive-label">
      <div class="col-sm-12 col-md-3"><label for="email">E-mail</label></div>
      <div class="col-sm-12 col-md"><input type="text" id="email" placeholder="E-mail"/></div>
    </div>
    <div class="row responsive-label">
      <div class="col-sm-12 col-md-12"><button>Register</button></div>
    </div>
  </fieldset>
</form>
</template>

<script>
export default {
    
}
</script>

<style scoped>

</style>