<template>
  <mark class="tag">{{ name }} <span v-if="editable">[x]</span></mark>
</template>

<script>
export default {
    props: {
        'name': String,
        'editable': Boolean
    },
}
</script>

<style scoped>
</style>
