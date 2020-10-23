<template>
  <div class="dashboard">
    <HelloWorld />
  </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import HelloWorld from '@/components/HelloWorld.vue'; // @ is an alias to /src

export default defineComponent({
  name: 'Dashboard',
  components: {
    HelloWorld,
  },
});
</script>
