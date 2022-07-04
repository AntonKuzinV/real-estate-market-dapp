<template>
  <div class="card">
    <b-button variant="dark"><b-icon icon="plus"/></b-button>
    <div class="card-body">
      <h5 class="card-title card-name">{{ TEXT }}</h5>
      <p class="card-text">Type: {{}}</p>
      <p class="card-text">Price: {{}}$</p>
      <p class="card-text">Rooms: {{}}</p>
    </div>
  </div>
</template>

<script>
export default {
  name: 'PropertyCard',
  props: {
    property: {
      type: Object,
      required: true,
    },
    contractId: {
      type: String,
      required: true,
    },
    buyProperty: {
      type: Function,
      required: true,
    },
    putPropertyOffSale: {
      type: Function,
      required: true,
    },
    putPropertyOnSale: {
      type: Function,
      required: true,
    },
    deleteProperty: {
      type: Function,
      required: true,
    }
  },
};
</script>

<style scoped>
.card {
  border: 1px solid #474747;
  transition: all ease-in-out 0.3s;
  background-color: #fff;
  margin-bottom: 20px;
  width: 100%;
}
.card img {
  max-height: 400px;
  min-height: 400px;
}
.card-body {
  position: absolute;
  text-align: left;
  left: 0;
  bottom: 38px;
  color: #ffffff;
  background-color: rgba(0, 0, 0, 0.5);
  width: 100%;
}
.card:hover {
  border-color: #fff;
  box-shadow: 0 0 25px 0 rgba(0, 0, 0, 0.5);
  transform: translateY(-10px);
}
</style>