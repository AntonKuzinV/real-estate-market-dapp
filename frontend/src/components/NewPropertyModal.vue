<template>
  <div class="modal fade" id="staticBackdrop" data-bs-backdrop="static" data-bs-keyboard="false" tabindex="-1"
       aria-labelledby="staticBackdropLabel" aria-hidden="true">
    <div class="modal-dialog">
      <div class="modal-content">
        <div class="modal-header">
          <h5 class="modal-title" id="staticBackdropLabel">New Property</h5>
          <button type="button" class="btn-close" data-bs-dismiss="modal" aria-label="Close"></button>
        </div>
        <div class="modal-body">
          <div class="form-floating mb-3">
            <input type="text" class="form-control" id="floatingNameInput" placeholder="Property Name"
                   v-model.trim="propertyName">
            <label for="floatingNameInput">Property Name</label>
          </div>
          <div class="form-floating mb-3">
            <select class="form-select" id="floatingTypeInput" v-model.trim="propertyType">
              <option value="Apartments">Apartments</option>
              <option value="House">House</option>
            </select>
            <label for="floatingTypeInput">Property Type</label>
          </div>
          <div class="form-floating mb-3">
            <input type="text" class="form-control" id="floatingLocationInput" placeholder="Property Location"
                   v-model.trim="propertyLocation">
            <label for="floatingLocationInput">Property Location</label>
          </div>
          <div class="form-floating mb-3">
            <input type="number" class="form-control" id="floatingRoomsInput" placeholder="Property Floor Level"
                   v-model.trim="propertyRooms">
            <label for="floatingFloorInput">Property Rooms</label>
          </div>
          <div class="form-floating mb-3">
            <input type="number" class="form-control" id="floatingFloorInput" placeholder="Property Floor Level"
                   v-model.trim="propertyFloor">
            <label for="floatingFloorInput">Property Floor Level</label>
          </div>
          <div class="form-floating mb-3">
            <input type="number" class="form-control" id="floatingStoreysInput" placeholder="Property Storeys"
                   v-model="propertyStoreys">
            <label for="floatingStoreysInput">Property Storeys</label>
          </div>
          <div class="form-floating mb-3">
            <input type="number" class="form-control" id="floatingSquarespaceInput" placeholder="Property Squarespace"
                   v-model="propertySquarespace">
            <label for="floatingSquarespaceInput">Property Squarespace</label>
          </div>
          <div class="form-floating mb-3">
            <input type="number" class="form-control" id="floatingPriceInput" placeholder="Property Price in Ⓝ"
                   v-model="propertyPrice">
            <label for="floatingPriceInput">Property Price in Ⓝ</label>
          </div>
        </div>
        <div class="modal-footer">
          <button type="button" class="btn btn-outline-dark" data-bs-dismiss="modal">Cancel</button>
          <button type="button" class="btn btn-dark" data-bs-dismiss="modal" @click="addNewProperty">Place it on market!</button>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
import { ref } from 'vue';

export default {
  name: 'NewPropertyModal',

  props: {
    addProperty: {
      typeof: Function,
      required: true,
    },
    updateProperties: {
      typeof: Function,
      required: true,
    },
  },
  setup(props) {
    const propertyName =  ref('');
    const propertyType =  ref('');
    const propertyLocation =  ref('');
    const propertyRooms =  ref(1);
    const propertyFloor =  ref(1);
    const propertyStoreys =  ref(1);
    const propertySquarespace =  ref(1);
    const propertyPrice =  ref(1);
    return {
      propertyName,
      propertyType,
      propertyLocation,
      propertyRooms,
      propertyFloor,
      propertyStoreys,
      propertySquarespace,
      propertyPrice,
      addNewProperty: async () => {
        await props.addProperty(propertyName.value, propertyType.value, propertyLocation.value, propertyRooms.value,
            propertyFloor.value, propertyStoreys.value, propertySquarespace.value, propertyPrice.value);
        await props.updateProperties();
      },
    };
  },
};
</script>

<style scoped>

</style>