<template>
  <div class="card text-white bg-dark mb-3 mx-1" style="max-width: 18rem;" ref="cardRef" @mouseenter="setGlow"
       @mouseleave="unsetGlow">
    <div class="card-header text-end">
      <span class="text-warning" v-if="!property[1].is_for_sale">CURRENTLY NOT FOR SALE</span>
      <span class="text-success" v-if="property[1].is_for_sale">FOR SALE</span>
    </div>
    <div class="card-body">
      <h4 class="card-title text-uppercase text-center">{{ property[1].property_name }}</h4>
      <h5 class="card-title"></h5>
      <h6 class="card-text text-start"> Floor: {{ property[1].floor }}</h6>
      <h6 class="card-text text-start"> Rooms: {{ property[1].rooms }}</h6>
      <h6 class="card-text text-start"> Storeys: {{ property[1].storeys }}</h6>
      <h6 class="card-text text-start"> Squarespace: {{ property[1].squarespace }}</h6>
      <div class="btn-group" role="group">
        <button type="button" class="btn btn-light btn-outline-success" v-if="accountId && property[1].is_for_sale && property[1].owner !== accountId"
                @click="buyProperty(property[0], property[1].price)">Buy property for {{ property[1].price }} Ⓝ
        </button>
        <button type="button" class="btn btn-light btn-outline-success"
                v-if="property[1].owner === accountId && !property[1].is_for_sale" @click="putOnSale(property[0])">Put
          on sale
        </button>
        <button type="button" class="btn btn-light btn-outline-warning"
                v-if="property[1].owner === accountId && property[1].is_for_sale" @click="putOffSale(property[0])">Put
          off sale
        </button>
        <button type="button" class="btn btn-light btn-outline-danger" v-if="property[1].owner === accountId">Delete
          property
        </button>
      </div>
    </div>
  </div>
</template>

<script>
import { useProperties } from '@/composables/near';

export default {
  name: 'PropertyCard',
  props: {
    property: {
      type: Object,
      required: true,
    },
    accountId: {
      type: String,
      required: false,
    },
    updateProperties: {
      type: Function,
      required: true,
    }
  },
  methods: {
    setGlow() {
      if (this.property[1].owner === this.accountId) {
        this.$refs.cardRef.style.boxShadow = '0 0 50px 0 rgb(255, 0, 0)';
      }
      if (this.property[1].owner !== this.accountId && this.property[1].is_for_sale) {
        this.$refs.cardRef.style.boxShadow = '0 0 50px 0 rgb(0, 255, 0)';
      }
      if (this.property[1].owner !== this.accountId && !this.property[1].is_for_sale) {
        this.$refs.cardRef.style.boxShadow = '0 0 50px 0 rgb(255, 255, 0)';
      }
    },
    unsetGlow() {
      this.$refs.cardRef.style.boxShadow = '0 0 50px 0 transparent';
    },
  },
  setup(props) {
    const {
      CONTRACT_ID,
      addProperty,
      deleteProperty,
      buyProperty,
      putPropertyOnSale,
      putPropertyOffSale,
    } = useProperties();

    return {
      CONTRACT_ID,
      addProperty,
      deleteProperty,
      buyProperty,
      putOnSale: async (id) => {
        await putPropertyOnSale(id);
        await props.updateProperties();
      },
      putOffSale: async (id) => {
        await putPropertyOffSale(id);
        await props.updateProperties();
      },
    };
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