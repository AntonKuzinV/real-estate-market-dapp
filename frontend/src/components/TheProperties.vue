<template>
  <div class="row row-cols-1 row-cols-md-2 g-4 ms-4">
    <PropertyCard v-for="property in properties" :key="property[0]"
                  :property="property" :account-id="accountId" :update-properties="updateProperties"/>
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
    accountId: {
      type: String,
      required: true,
    },
    updateProperties: {
      type: Function,
      required: true,
    }
  }
};
</script>

<style scoped>

</style>