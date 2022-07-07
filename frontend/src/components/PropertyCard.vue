<template>
  <div class="card text-white bg-dark mb-3 mx-1" style="max-width: 18rem;" ref="cardRef" @mouseenter="setGlow" @mouseleave="unsetGlow">
    <div class="card-header">{{ property[1].property_type}}</div>
    <div class="card-body">
      <h4 class="card-title text-uppercase text-center">{{ property[1].property_name }}</h4>
      <h5 class="card-title"></h5>
      <h6 class="card-text text-end"> Floor: {{ property[1].floor }}</h6>
      <h6 class="card-text text-end"> Rooms: {{ property[1].rooms }}</h6>
      <h6 class="card-text text-end"> Storeys: {{ property[1].storeys }}</h6>
      <div class="btn-group" role="group">
        <button type="button" class="btn btn-light btn-outline-success" v-if="property[1].owner !== accountId">Buy property</button>
        <button type="button" class="btn btn-light btn-outline-success" v-if="property[1].owner === accountId && !property[1].is_for_sale">Put
          on sale
        </button>
        <button type="button" class="btn btn-light btn-outline-warning" v-if="property[1].owner === accountId && property[1].is_for_sale">Put
          off sale
        </button>
        <button type="button" class="btn btn-light btn-outline-danger" v-if="property[1].owner === accountId">Delete property</button>
      </div>
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
    },
    accountId: {
      typeof: String,
      required: true,
    },
  },
  methods: {
    setGlow() {
      if (this.property[1].owner === this.accountId) {
        this.$refs.cardRef.style.boxShadow = "0 0 50px 0 rgb(255, 21, 21)";
      } else {
        this.$refs.cardRef.style.boxShadow = "0 0 50px 0 rgb(81, 173, 45)";
      }
    },
    unsetGlow() {
      this.$refs.cardRef.style.boxShadow = "0 0 50px 0 transparent";
    }
  },
};
</script>

<style scoped>
.card:hover {
  border-color: #000;
  /*box-shadow: 0 0 50px 0 rgb(81, 173, 45);*/
  transform: translateY(-10px);
  transition-duration: 0.5s;
}
</style>