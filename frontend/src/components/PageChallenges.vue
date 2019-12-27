<template>
  <div>
    <challenge-filter />
    <div class="row">
      <challenge-card name="Cool challenge" description="Description of pretty cool challenge" v-bind:tags="['pwn', 'crypto']" />
      <challenge-card name="Difficult challenge" description="Description of difficult challenge" v-bind:tags="['pwn']" />
      <challenge-card name="Mad challenge" description="Description of pretty mad challenge" v-bind:tags="['pwn']" />
      <challenge-card name="Easy challenge" description="Description of pretty east challenge" v-bind:tags="['pwn', 'heap']" />
      <challenge-card name="Tricky challenge" description="Description of pretty tricky challenge" v-bind:tags="['pwn']" />
    </div>
  </div>
</template>

<script>
import ChallengeCard from './ChallengeCard.vue'
import ChallengeFilter from './ChallengeFilter.vue'

export default {
    components: {
      ChallengeCard,
      ChallengeFilter,
    }
}
</script>

<style scoped>

</style>
