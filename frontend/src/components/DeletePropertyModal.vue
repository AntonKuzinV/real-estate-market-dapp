<template>
  <div class="modal fade" id="deletePropertyModal" tabindex="-1" aria-labelledby="deletePropertyModal" aria-hidden="true">
    <div class="modal-dialog">
      <div class="modal-content">
        <div class="modal-header">
          <h5 class="modal-title" id="deletePropertyModalLabel">Delete Property</h5>
          <button type="button" class="btn-close" data-bs-dismiss="modal" aria-label="Close"></button>
        </div>
        <div class="modal-body">
          Are you sure you want to delete this property?
        </div>
        <div class="modal-footer">
          <button type="button" class="btn btn-outline-dark" data-bs-dismiss="modal">Cancel</button>
          <button type="button" class="btn btn-dark" data-bs-dismiss="modal" @click="deletePropertyById">Delete</button>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
export default {
  name: 'DeletePropertyModal',
  props: {
    deleteProperty: {
      typeof: Function,
      required: true,
    },
    updateProperties: {
      typeof: Function,
      required: true,
    },
    propertyId: {
      typeof: Number,
      required: true,
    }
  },
  setup(props) {
    return {
      deletePropertyById: async () => {
        await props.deleteProperty(props.propertyId);
        await props.updateProperties();
      }
    }
  }
};
</script>

<style scoped>

</style>