import json

import crusty_requests

import asyncio


async def main():
	req = await crusty_requests.get(url="https://rest.ensembl.org/info/ping?content-type=application/json")
	data = json.dumps(await req.json)
	print(data)

asyncio.run(main())
