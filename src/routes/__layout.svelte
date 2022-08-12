<script>
	import 'carbon-components-svelte/css/g90.css';
	import {
		Header,
		SideNav,
		SideNavItems,
		SideNavLink,
		SideNavDivider,
		SkipToContent,
		Content,
		Grid,
		Row,
		Column
	} from 'carbon-components-svelte';
	import Fade from 'carbon-icons-svelte/lib/Fade.svelte';

	let isSideNavOpen = false;
</script>

<Header company="OpenGOAL" platformName="Launcher" bind:isSideNavOpen>
	<svelte:fragment slot="skip-to-content">
		<SkipToContent />
	</svelte:fragment>

	<SideNav bind:isOpen={isSideNavOpen} rail>
		<SideNavItems>
			<SideNavLink icon={Fade} text="Jak and Daxter: TPL" href="/game/jak1" isSelected />
			<SideNavLink icon={Fade} text="Jak 2" href="/game/jak2" />
			<SideNavLink icon={Fade} text="Jak 3" href="/game/jak3" />
			<SideNavDivider />
			<SideNavLink icon={Fade} text="FAQ" href="/faq" />
			<SideNavLink icon={Fade} text="Settings" href="/settings" />
		</SideNavItems>
	</SideNav>
</Header>

<Content>
	<Grid>
		<Row>
			<Column>
				<slot />
			</Column>
		</Row>
	</Grid>
</Content>
