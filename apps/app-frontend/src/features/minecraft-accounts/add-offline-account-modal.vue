<template>
	<NewModal ref="modal" :header="formatMessage(messages.title)" max-width="440px" width="100%">
		<div class="flex flex-col gap-4">
			<p class="m-0 text-primary">{{ formatMessage(messages.description) }}</p>
			<div class="labeled_input w-full">
				<p class="text-contrast font-semibold">{{ formatMessage(messages.usernameLabel) }}</p>
				<Input
					v-model="username"
					type="text"
					:placeholder="formatMessage(messages.usernamePlaceholder)"
					wrapper-class="w-full"
					@keydown.enter="create"
				/>
			</div>
			<p v-if="usernameInvalid" class="m-0 text-red">
				{{ formatMessage(messages.usernameRules) }}
			</p>
			<p v-else-if="error" class="m-0 text-red">{{ error }}</p>
			<div class="flex gap-2 justify-end">
				<Button @click="modal?.hide()">{{ formatMessage(commonMessages.cancelButton) }}</Button>
				<Button
					type="colored"
					color="brand"
					:disabled="!usernameValid || creating"
					@click="create"
				>
					<SpinnerIcon v-if="creating" class="animate-spin" />
					{{ formatMessage(messages.createButton) }}
				</Button>
			</div>
		</div>
	</NewModal>
</template>

<script setup lang="ts">
import { SpinnerIcon } from '@modrinth/assets'
import { Button, commonMessages, defineMessages, Input, NewModal, useVIntl } from '@modrinth/ui'
import { computed, ref } from 'vue'

import { trackEvent } from '@/helpers/analytics'
import { add_offline_account } from '@/helpers/auth'

type OfflineAccount = {
	profile: {
		id: string
		name: string
	}
	offline?: boolean
}

const emit = defineEmits<{
	created: [account: OfflineAccount]
}>()

const { formatMessage } = useVIntl()
const modal = ref<InstanceType<typeof NewModal> | null>(null)
const username = ref('')
const error = ref<string | undefined>()
const creating = ref(false)
const usernameValid = computed(() => /^[A-Za-z0-9_]{3,16}$/.test(username.value.trim()))
const usernameInvalid = computed(
	() => username.value.trim().length > 0 && !usernameValid.value,
)

function show() {
	error.value = undefined
	modal.value?.show()
}

async function create() {
	if (!usernameValid.value || creating.value) return

	creating.value = true
	error.value = undefined

	try {
		const account = await add_offline_account(username.value.trim())
		modal.value?.hide()
		username.value = ''
		await trackEvent('AccountLogIn', { source: 'AddOfflineAccountModal' })
		if (account) emit('created', account)
	} catch (caught) {
		error.value =
			typeof caught === 'string' ? caught : ((caught as Error)?.message ?? String(caught))
	} finally {
		creating.value = false
	}
}

defineExpose({ show })

const messages = defineMessages({
	title: {
		id: 'minecraft-account.offline-title',
		defaultMessage: 'Add offline account',
	},
	description: {
		id: 'minecraft-account.offline-description',
		defaultMessage:
			'Offline accounts don’t need a Microsoft account. They can play in singleplayer and on offline-mode servers, but online-mode servers will not let them join.',
	},
	usernameLabel: {
		id: 'minecraft-account.offline-username-label',
		defaultMessage: 'Username',
	},
	usernamePlaceholder: {
		id: 'minecraft-account.offline-username-placeholder',
		defaultMessage: 'Steve',
	},
	usernameRules: {
		id: 'minecraft-account.offline-username-rules',
		defaultMessage:
			'Usernames must be 3-16 characters and can only use letters, numbers and underscores.',
	},
	createButton: {
		id: 'minecraft-account.offline-create',
		defaultMessage: 'Create account',
	},
})
</script>
