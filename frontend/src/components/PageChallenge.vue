<template>
  <div>
    <pre>ID: {{ $route.params.id }}</pre>
    <h1>{{ title }}</h1>
    <div>{{ description }}</div>
    <div v-if="connect">{{ connect }}</div>
    <div v-if="download">{{ download }}</div>
    <div>
        Status: {{ status }}
        <div v-if="status == 0">
            <button>Start challenge</button>
        </div>
    </div>
  </div>
</template>

<script>
const CHALLENGE_STATUS = {
    OFFLINE: 0,
    STARTING: 1,
    ONLINE: 2,
    SHUTTING_DOWN: 3,
    ERROR: 4,
}

export default {
    components: {
    },

    data: () => { return {
      title: "Challenge Title",
      description: "The description of the challenge",
      status: CHALLENGE_STATUS.OFFLINE,
      download: "download-link",
      connect: "nc 127.0.0.1 31337",
    };}
}
</script>

<style scoped>

</style>
