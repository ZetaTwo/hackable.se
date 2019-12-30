<template>
  <mark class="tag"><slot></slot><span v-if="editable"> [x]</span></mark>
</template>

<script>
export default {
    props: {
        'editable': Boolean
    },
}
</script>

<style scoped>
</style>
