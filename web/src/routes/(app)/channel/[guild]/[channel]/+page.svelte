<script lang="ts">
	import { i18n } from '$/i18n';
	import { Helper, Message } from '$/sdk';
	import { page } from '$app/stores';
	import { user, guilds, messageDraft } from '$/state';
	import { i18nDateFormats } from '$/utils/date';
	import UserInfo from '$/components/UserInfo.svelte';

	$: guild = $guilds?.find((v) => v.id === $page.params.guild);
	$: channel = guild?.channels?.find((v) => v.id === $page.params.channel);

	$: draftMessage = messageDraft($page.params.channel);

	// TODO: Add error handling.
	let messages: Promise<Message.GetResponse | undefined>;
	$: messages = Promise.resolve(undefined);
	async function fetchMessages() {
		if (!$user) return;
		messages = Message.get($user.accessToken, {
			channelId: $page.params.channel
		})
			.emptyOnError()
			.send();
	}
	$: {
		$user;
		$page.params.channel;
		fetchMessages();
	}

	async function sendMessage() {
		if (!$user) return;
		const messageIdPromise = Message.send($user.accessToken, {
			channelId: $page.params.channel,
			content: $draftMessage
		})
			.emptyOnError()
			.send();
		$draftMessage = '';
		const messageId = await messageIdPromise;

		if (messageId) {
			console.log('sent');
		}
	}

	function formatDate(utcSeconds: number) {
		// `utcSeconds` is the seconds in UTC since 2023/01/01 00:00:00
		const ourEpoch = new Date('2023').getTime();
		const date = new Date(utcSeconds + ourEpoch);
		// If it was today, return the today format
		// If it was yesterday, return the yesterday format
		// Else, return the normal format
		const dateAtMidnight = Math.floor(date.getTime() / 86400000);
		const todayAtMidnight = Math.floor(Date.now() / 86400000);
		if (todayAtMidnight === dateAtMidnight) {
			return i18n('DATE_FORMAT_TODAY', i18nDateFormats(date));
		} else if (todayAtMidnight - 1 === dateAtMidnight) {
			return i18n('DATE_FORMAT_YESTERDAY', i18nDateFormats(date));
		} else {
			return i18n('DATE_FORMAT', i18nDateFormats(date));
		}
	}
</script>

<div class="flex w-60 shrink-0 flex-col border-r border-zinc-900">
	<div class="flex h-12 shrink-0 items-center border-b border-zinc-900 px-6">
		<span class="text-lg font-bold text-zinc-100">{guild?.name}</span>
	</div>
	<div class="mt-2 flex grow flex-col overflow-scroll">
		{#each guild?.channels || [] as channel}
			{@const selected = channel.id === $page.params.channel}
			<a href={`/channel/${guild?.id}/${channel.id}`} class="group px-2 py-0.5">
				<div
					class="flex h-9 items-center gap-2 rounded px-2 {selected
						? 'bg-zinc-600'
						: 'group-hover:bg-zinc-700'}"
				>
					<span class="text-xl font-bold italic {selected ? 'text-zinc-300' : 'text-zinc-500'}">
						#
					</span>
					<span class={selected ? 'text-zinc-100' : 'text-zinc-400'}>{channel.name}</span>
				</div>
			</a>
		{/each}
	</div>

	<UserInfo />
</div>

<div class="flex h-screen grow flex-col">
	<div class="flex h-12 shrink-0 items-center gap-2 px-4">
		<span class="text-xl font-bold italic text-zinc-300">#</span>
		<span class="text-lg font-bold text-zinc-100">{channel?.name}</span>
	</div>
	<div class="flex grow flex-col overflow-scroll bg-zinc-900">
		<div class="flex grow flex-col-reverse overflow-scroll pt-60">
			{#await messages}
				<p class="mx-4 my-2 animate-pulse text-xl font-bold text-white">Loading...</p>
			{:then msgs}
				{#each msgs ?? [] as msg}
					<div class="flex flex-row items-start gap-4 px-4 py-2">
						<div class="aspect-square w-10">
							<img
								src={Helper.profileImage(msg.author)}
								class="h-full w-full rounded-full bg-white"
								alt="{msg.author.username}'s Avatar"
							/>
						</div>
						<div class="flex flex-col">
							<div class="flex flex-row items-baseline gap-2">
								<span class="font-bold text-zinc-100">{msg.author.username}</span>
								<span class="text-xs text-zinc-500">{formatDate(msg.sent_at)}</span>
							</div>
							<div class="flex flex-row">
								<span class="text-zinc-300">{msg.content}</span>
							</div>
						</div>
					</div>
				{/each}
			{/await}
		</div>
		<div class="shrink-0 px-4 py-4">
			<input
				type="text"
				class="h-full w-full rounded-lg bg-zinc-700 px-4 py-2 text-white ring-0 ring-transparent transition-all placeholder:text-zinc-500 focus:outline-none focus:ring-2 focus:ring-amber-200"
				placeholder="Message #{channel?.name}"
				bind:value={$draftMessage}
				on:keydown={async (e) => {
					if (e.key === 'Enter' && !e.shiftKey) {
						await sendMessage();
						fetchMessages();
					}
				}}
			/>
		</div>
	</div>
</div>
