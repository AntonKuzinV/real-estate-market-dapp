<template>
  <div class="row row-cols-1 row-cols-md-2 g-4 ms-4">
    <PropertyCard v-for="property in properties" :key="property[0]"
                  :property="property" :delete-property="deleteProperty" :put-property-on-sale="putPropertyOffSale"
                  :put-property-off-sale="putPropertyOffSale" :buy-property="buyProperty" :contract-id="contractId"
                  :account-id="accountId"/>
  </div>
</template>

<script>
import PropertyCard from '@/components/PropertyCard';

export default {
  name: 'TheProperties',
  components: { PropertyCard },
  props: {
    properties: {
      type: Array,
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
};
</script>

<style scoped>

</style>