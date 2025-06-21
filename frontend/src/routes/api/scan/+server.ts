import { redirect, type RequestHandler } from '@sveltejs/kit';

const QRBOT_SCAN_URL = "https://qrbot.net/x-callback-url/scan?x-success=";

export const GET: RequestHandler = async ({ fetch, url }) => {
  const shouldRedirect = url.searchParams.get("redirect");

  // qrbot reference
  // https://ioshelp.qrbot.net/xcallback
  
  if (shouldRedirect === "true") {
    const redirectUrl = url.origin + "/api/scan";
    throw redirect(302, QRBOT_SCAN_URL + redirectUrl) 
  }

  const source = url.searchParams.get("x-source")
  const content = url.searchParams.get("content")
  const format = url.searchParams.get("format")
  
  if (source === "qrbot" && format === "ean13") {
    throw redirect(302, "/add?isbn=" + content)
  }

  return new Response(JSON.stringify({
      source, content, format,
  }));
};
