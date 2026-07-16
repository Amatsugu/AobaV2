namespace AobaCore.Tools;

public record AvifColorInfo(
	string PixFmt,
	string ColorTransfer,
	string ColorPrimaries,
	string ColorSpace,
	string ColorRange)
{
	public bool IsHdr => ColorTransfer.Equals("smpte2084", StringComparison.OrdinalIgnoreCase)
		|| ColorTransfer.Equals("arib-std-b67", StringComparison.OrdinalIgnoreCase);

	public override string ToString()
	{
		return $"pix_fmt={PixFmt} transfer={ColorTransfer} primaries={ColorPrimaries} space={ColorSpace} range={ColorRange} ({(IsHdr ? $"HDR: {ColorTransfer}" : "SDR/untagged")})";
	}
}