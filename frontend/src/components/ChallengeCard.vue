<template>
  <div class="card">
    <div class="section"><h3>{{ name }}</h3></div>
    <div class="section">{{ description }}</div>
    <div class="section"><tag v-for="tag in tags" v-bind:key="tag">{{ tag }}</tag></div>
  </div>
</template>

<script>
import Tag from './Tag.vue'

export default {
    props: {
        'name': String,
        'description': String,
        'tags': Array
    },
    components: {
        Tag
    }
}
</script>

<style scoped>
</style>
