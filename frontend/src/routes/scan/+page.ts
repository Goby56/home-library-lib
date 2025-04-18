import type { PageLoad } from './$types';
import { redirect } from '@sveltejs/kit';

export const load: PageLoad = ({ url }) => {
  const shouldRedirect = url.searchParams.get("redirect");
  
  // qrbot reference
  // https://ioshelp.qrbot.net/xcallback
  const source = url.searchParams.get("source")
  const content = url.searchParams.get("content")
  const format = url.searchParams.get("format")

  if (shouldRedirect === "true") {
    throw redirect(302, "https://qrbot.net/x-callback-url/scan?x-success=http://192.168.1.223:5173/scan");
  }

  return {
      source: source,
      content: content,
      format: format
  };
};

