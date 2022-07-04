<template>
  <div class="container">
    <header class="d-flex flex-wrap align-items-center justify-content-center justify-content-md-between py-3 mb-4 border-bottom">
      <a href="/" class="d-flex align-items-center col-md-3 mb-2 mb-md-0 text-dark text-decoration-none">
        <img src="https://upload.wikimedia.org/wikipedia/commons/thumb/d/d5/Rust_programming_language_black_logo.svg/2048px-Rust_programming_language_black_logo.svg.png" alt="" width="48" height="48">
        EAL ESTATE MARKET
      </a>

      <ul class="nav col-12 col-md-auto mb-2 justify-content-center mb-md-0">
        <li><span class="px-2 link-dark">Market</span></li>
        <li><span class="px-2 link-dark">Own property</span></li>
      </ul>

      <div class="col-md-3 text-end">
        <button v-if="!accountId" type="button" class="btn btn-outline-dark me-2" @click="signIn">Log in with NEAR Wallet</button>
        <span v-if="accountId" class="me-2">Hello, {{accountId}}</span>
        <button v-if="accountId" type="button" class="btn btn-outline-dark me-2" @click="signOut">Log out</button>
      </div>
    </header>
  </div>
</template>

<script>
export default {
  name: 'TheHeader',
  props: {
    accountId: {
      typeof:String,
      required:true
    },
    signIn: {
      typeof:Function,
      required:true
    },
    signOut: {
      typeof:Function,
      required:true
    },
  }
};
</script>

<style scoped>

</style>