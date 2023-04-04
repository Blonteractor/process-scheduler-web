<script lang="ts">
import client from "@/client";
import { defineComponent } from "vue";

export default defineComponent({
	props: {
		msg: String,
	},
	data() {
		return {
			serverMsg: this.msg != undefined ? this.msg : "kekw",
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
