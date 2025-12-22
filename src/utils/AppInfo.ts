export function channelWrapper(channel: string) {
    channel = channel.toLowerCase();
    const _channel = {
        overworld: "Main.More/About.UpdateChannel.Channels.Overworld",
        nether: "Main.More/About.UpdateChannel.Channels.Nether",
        ender: "Main.More/About.UpdateChannel.Channels.Ender",
    };
    return channel in _channel
        ? _channel[channel as keyof typeof _channel]
        : "Main.More/About.UpdateChannel.Channels.Unknown";
}
