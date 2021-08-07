package compression

import (
	"bytes"
	"compress/zlib"
	"encoding/json"
	"fmt"
	"io"

	"github.com/rs/zerolog/log"
	gatewayTypes "github.com/discordnova/nova/common/discord/types/payloads/gateway"
	"github.com/discordnova/nova/common/gateway"
)

// JsonZlibCompressor is the default compression interface.
type JsonZlibCompressor struct {
	buffer *bytes.Buffer
	reader io.ReadCloser
}

// NewJsonZlibCompressor creates an instance of JsonZlibCompressor
func NewJsonZlibCompressor() gateway.Compression {
	return &JsonZlibCompressor{
		buffer: bytes.NewBuffer([]byte{}),
	}
}

func (compressor *JsonZlibCompressor) Reset() error {
	compressor.buffer.Reset()
	if compressor.reader == nil {
		return nil
	}
	err := compressor.reader.Close()
	if err != nil {
		return err
	}
	compressor.reader = nil
	return nil
}

// GetConnectionOptions gets the required options for the gateway.
func (compressor JsonZlibCompressor) GetConnectionOptions() gateway.GatewayConnectionOptions {
	// Gateway options for the discord gateway.
	return gateway.GatewayConnectionOptions{
		Encoding:             "json",
		TransportCompression: "zlib-stream",
	}
}

// DecodeMessage decodes messages using the compressor.
func (compressor *JsonZlibCompressor) DecodeMessage(data []byte) (*gatewayTypes.Payload, error) {

	// check if the message have the zlib suffix to avoid ruining our zlib context :'(
	if !bytes.HasSuffix(data, []byte{0x00, 0x00, 0xff, 0xff}) {
		return nil, fmt.Errorf("the gateway failed to verify the message validity due to invalid suffix")
	}

	// add the data to the buffer for the decompression.
	compressor.buffer.Write(data)

	// we can't create the reader without data, so we initialize on the first decompression.
	if compressor.reader == nil {
		reader, err := zlib.NewReader(compressor.buffer)
		if err != nil {
			log.Err(err).Msgf("Failed to initialize zlib reader")
		}
		compressor.reader = reader
	}

	// we unmarshal the reader as json
	inter := gatewayTypes.Payload{}
	decoder := json.NewDecoder(compressor.reader)
	err := decoder.Decode(&inter)

	if err != nil {
		// the unmarshalling failed
		return nil, err
	}

	return &inter, nil
}
