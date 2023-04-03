<script lang="ts">
// import client from "@/client";
import { defineComponent } from "vue";
import { client } from "@/client";

export default defineComponent({
	props: {
		msg: { type: String, required: true },
	},
	data() {
		return {
			serverMsg: this.msg,
		};
	},
	async mounted() {
		this.serverMsg = await client
			.echo({ text: this.msg })
			.then((response) => response.text);
	},
});
</script>

<template>
	<h1 class="green">{{ serverMsg }}</h1>
</template>

<style scoped>
h1 {
	font-weight: 500;
	font-size: 2.6rem;
	top: -10px;
}
</style>
